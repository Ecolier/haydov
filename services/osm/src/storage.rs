use bytes::BytesMut;
use async_trait::async_trait;
use bytes::Bytes;
use futures_util::StreamExt;
use minio::s3::{
    builders::CompleteMultipartUpload,
    http::BaseUrl,
    response::CreateMultipartUploadResponse,
    segmented_bytes::SegmentedBytes,
    types::{PartInfo, S3Api},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("S3 error")]
    S3(#[from] minio::s3::error::Error),
}

#[async_trait]
pub trait StorageBackend: Send + Sync {
    async fn stream_multipart<S>(
        &self,
        bucket: &str,
        object: &str,
        mut stream: S,
        total_size: Option<u64>,
        chunk_size: usize,
    ) -> Result<CompleteMultipartUpload, minio::s3::error::Error>
    where
        S: StreamExt<Item = Result<Bytes, reqwest::Error>> + Send + Unpin;
}

#[async_trait]
impl StorageBackend for minio::s3::Client {
    async fn stream_multipart<S>(
        &self,
        bucket: &str,
        object: &str,
        mut stream: S,
        total_size: Option<u64>,
        chunk_size: usize,
    ) -> Result<CompleteMultipartUpload, minio::s3::error::Error>
    where
        S: StreamExt<Item = Result<Bytes, reqwest::Error>> + Send + Unpin,
    {
        let mut buffer = BytesMut::with_capacity(chunk_size);
        let mut parts_count = 1u16;

        let parts_len = total_size.map(|size: u64| (size / chunk_size as u64 + 1) as usize);;
        let mut parts: Vec<PartInfo> = Vec::with_capacity(parts_len.unwrap_or(chunk_size));

        let CreateMultipartUploadResponse { upload_id, .. } =
            self.create_multipart_upload(bucket, object).send().await?;

        // Process the stream of bytes, writing them to the file
        // and updating the downloaded size.
        while let Some(item) = stream.next().await {
            let chunk = item?;
            buffer.extend_from_slice(&chunk);

            // If the buffer has enough data, upload it as a part.
            if buffer.len() >= chunk_size {
                let part_bytes = buffer.split_to(chunk_size).freeze();
                let part_resp = self
                    .upload_part(
                        bucket,
                        object,
                        &upload_id,
                        parts_count,
                        SegmentedBytes::from(part_bytes),
                    )
                    .send()
                    .await?;
                parts.push(PartInfo {
                    number: parts_count,
                    size: chunk_size as u64,
                    etag: part_resp.etag,
                });
                parts_count += 1;
            }
        }

        // If there are remaining bytes in the buffer, upload them as the last part.
        // This ensures that any remaining data is uploaded, even if it's less than PART_SIZE.
        if !buffer.is_empty() {
            let final_part = buffer.clone().freeze();
            let part_resp = self
                .upload_part(
                    bucket,
                    object,
                    &upload_id,
                    parts_count,
                    SegmentedBytes::from(final_part),
                )
                .send()
                .await?;
            parts.push(PartInfo {
                number: parts_count,
                size: buffer.len() as u64,
                etag: part_resp.etag,
            });
        }

        // Complete the multipart upload
        // This finalizes the upload and makes the object available in the S3 bucket.
        return Ok(self.complete_multipart_upload(bucket, object, upload_id, parts));
    }
}

pub async fn init<'a>(
    base_url: &'a str,
    access_key: &'a str,
    secret_key: &'a str,
    bucket_name: &'a str,
) -> Result<(minio::s3::Client, &'a str), StorageError> {
    let static_provider = minio::s3::creds::StaticProvider::new(access_key, secret_key, None);
    let base_url = base_url.parse::<BaseUrl>()?;
    let client = minio::s3::ClientBuilder::new(base_url)
        .provider(Some(Box::new(static_provider.clone())))
        .build()?;
    let bucket_name = match client.create_bucket(bucket_name).send().await {
        Ok(_) => bucket_name,
        Err(e) => match &e {
            minio::s3::error::Error::S3Error(_) => bucket_name,
            _ => {
                return Err(StorageError::S3(e));
            }
        },
    };
    Ok((client, bucket_name))
}
