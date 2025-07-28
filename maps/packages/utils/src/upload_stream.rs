use aws_sdk_s3::{
    Client,
    primitives::ByteStream,
    types::{CompletedMultipartUpload, CompletedPart},
};
use bytes::{Bytes, BytesMut};
use futures::{Stream, StreamExt};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("S3 create multipart upload error")]
    S3CreateMultipartError(
        #[from]
        aws_sdk_s3::error::SdkError<
            aws_sdk_s3::operation::create_multipart_upload::CreateMultipartUploadError,
        >,
    ),

    #[error("S3 upload part error")]
    S3UploadPartError(
        #[from] aws_sdk_s3::error::SdkError<aws_sdk_s3::operation::upload_part::UploadPartError>,
    ),

    #[error("S3 complete multipart upload error")]
    S3CompleteMultipartError(
        #[from]
        aws_sdk_s3::error::SdkError<
            aws_sdk_s3::operation::complete_multipart_upload::CompleteMultipartUploadError,
        >,
    ),

    #[error("Stream error")]
    StreamError(#[from] Box<dyn std::error::Error + Send + Sync>),

    #[error("Missing required parameter: {0}")]
    MissingParameter(String),
}

pub trait S3ClientExt {
    fn bucket(&self, name: impl Into<String>) -> BucketBuilder;
}

impl S3ClientExt for Client {
    fn bucket(&self, name: impl Into<String>) -> BucketBuilder {
        BucketBuilder::new(self.clone(), name.into())
    }
}

pub struct BucketBuilder {
    client: Client,
    bucket_name: String,
}

impl BucketBuilder {
    fn new(client: Client, bucket_name: String) -> Self {
        Self {
            client,
            bucket_name,
        }
    }

    pub fn upload_stream_object(&self) -> UploadStreamBuilder {
        UploadStreamBuilder::new(self.client.clone(), self.bucket_name.clone())
    }
}

pub struct UploadStreamBuilder {
    client: Client,
    bucket_name: String,
    key: Option<String>,
    chunk_size: usize,
}

impl UploadStreamBuilder {
    fn new(client: Client, bucket_name: String) -> Self {
        Self {
            client,
            bucket_name,
            key: None,
            chunk_size: 5 * 1024 * 1024, // 5MB default
        }
    }

    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }

    pub fn chunk_size(mut self, size: usize) -> Self {
        self.chunk_size = size;
        self
    }

    pub async fn send<S>(self, stream: S) -> Result<(), Error>
    where
        S: Stream<Item = Result<Bytes, Box<dyn std::error::Error + Send + Sync>>> + Send + Unpin,
    {
        let key = self
            .key
            .as_ref()
            .ok_or_else(|| Error::MissingParameter("key".to_string()))?
            .clone();

        self.upload_multipart_stream(key, stream).await
    }

    async fn upload_multipart_stream<S>(&self, key: String, mut stream: S) -> Result<(), Error>
    where
        S: Stream<Item = Result<Bytes, Box<dyn std::error::Error + Send + Sync>>> + Send + Unpin,
    {
        let multipart_upload = self
            .client
            .create_multipart_upload()
            .bucket(&self.bucket_name)
            .key(&key)
            .send()
            .await?;

        let upload_id = multipart_upload.upload_id().unwrap_or_default();
        let mut buffer = BytesMut::with_capacity(self.chunk_size);
        let mut part_number = 1i32;
        let mut completed_parts = Vec::new();

        while let Some(chunk) = stream.next().await {
            let bytes = chunk?;
            buffer.extend_from_slice(&bytes);

            if buffer.len() >= self.chunk_size {
                let part_bytes = buffer.split_to(self.chunk_size).freeze();
                let part = self
                    .upload_part(&key, upload_id, part_number, part_bytes)
                    .await?;
                completed_parts.push(part);
                part_number += 1;
            }
        }

        // Upload remaining bytes
        if !buffer.is_empty() {
            let final_part = buffer.freeze();
            let part = self
                .upload_part(&key, upload_id, part_number, final_part)
                .await?;
            completed_parts.push(part);
        }

        // Complete multipart upload
        let completed_upload = CompletedMultipartUpload::builder()
            .set_parts(Some(completed_parts))
            .build();

        self.client
            .complete_multipart_upload()
            .bucket(&self.bucket_name)
            .key(&key)
            .multipart_upload(completed_upload)
            .upload_id(upload_id)
            .send()
            .await?;

        println!("Multipart upload completed for object: {}", key);
        Ok(())
    }

    async fn upload_part(
        &self,
        key: &str,
        upload_id: &str,
        part_number: i32,
        data: Bytes,
    ) -> Result<CompletedPart, Error> {
        let response = self
            .client
            .upload_part()
            .bucket(&self.bucket_name)
            .key(key)
            .part_number(part_number)
            .body(ByteStream::from(data))
            .upload_id(upload_id)
            .send()
            .await?;

        Ok(CompletedPart::builder()
            .part_number(part_number)
            .set_e_tag(response.e_tag().map(String::from))
            .build())
    }
}
