use bytes::{Bytes, BytesMut};
use futures_util::Stream;
use minio::s3::types::PartInfo;

pub async fn stream<S, E>(
        &self,
        stream: S,
        total_size: u64,
        chunk_size: usize,
    ) where
        S: Stream<Item = Bytes>,
    {
        let mut buffer = BytesMut::with_capacity(chunk_size);
        let mut parts_count = 1u16;
        let parts_len = (total_size / chunk_size as u64 + 1) as usize;
        let mut parts: Vec<PartInfo> = Vec::with_capacity(parts_len);

        let upload_id = match self
            .s3_client
            .create_multipart_upload(&self.osm_bucket, filename)
            .send()
            .await
        {
            Ok(response) => response.upload_id,
            Err(e) => {
                eprintln!(
                    "Failed to create multipart upload for '{}': {}",
                    filename, e
                );
                return;
            }
        };

        // Process the stream of bytes, writing them to the file
        // and updating the downloaded size.
        while let Some(item) = stream.next().await {
            let chunk = item
                .or(Err(format!("Error while downloading file")))
                .unwrap();

            buffer.extend_from_slice(&chunk);

            // If the buffer has enough data, upload it as a part.
            if buffer.len() >= chunk_size {
                let part_bytes = buffer.split_to(chunk_size).freeze();

                let part_resp = match s3_client
                    .upload_part(
                        &osm_bucket,
                        filename,
                        &upload_id,
                        parts_count,
                        SegmentedBytes::from(part_bytes),
                    )
                    .send()
                    .await
                {
                    Ok(resp) => resp,
                    Err(e) => {
                        eprintln!("Failed to upload part {}: {}", parts_count, e);
                        return;
                    }
                };

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
            println!(
                "Uploading final part {} of '{}' with size {} bytes",
                parts_count,
                filename,
                buffer.len()
            );
            let final_part = buffer.clone().freeze();
            let part_resp = match s3_client
                .upload_part(
                    &osm_bucket,
                    filename,
                    &upload_id,
                    parts_count,
                    SegmentedBytes::from(final_part),
                )
                .send()
                .await
            {
                Ok(resp) => resp,
                Err(e) => {
                    eprintln!("Failed to upload part {}: {}", parts_count, e);
                    return;
                }
            };

            parts.push(PartInfo {
                number: parts_count,
                size: buffer.len() as u64,
                etag: part_resp.etag,
            });
        }

        // Complete the multipart upload
        // This finalizes the upload and makes the object available in the S3 bucket.
        let complete_resp = match s3_client
            .complete_multipart_upload(&osm_bucket, filename, &upload_id, parts)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => {
                eprintln!(
                    "Failed to complete multipart upload for '{}': {}",
                    filename, e
                );
                return;
            }
        };
        println!(
            "Multipart upload for '{}' completed successfully: {:?}",
            filename, complete_resp.object
        );
    }

impl Storage {
    pub fn new() -> Self {
        
        Storage { s3_client }
    }

    pub async fn init(&self) {
        // Check if the bucket exists, and create it if it does not.
        // If the bucket already exists, it will return an error that we can handle.
        let osm_bucket = match self
            .s3_client
            .create_bucket(osm_bucket_name.clone())
            .send()
            .await
        {
            Ok(_) => {
                println!("Bucket '{}' created successfully.", osm_bucket_name);
                osm_bucket_name
            }
            Err(e) => match &e {
                minio::s3::error::Error::S3Error(_) => {
                    println!("Bucket '{}' already exists.", osm_bucket_name);
                    osm_bucket_name
                }
                _ => {
                    eprintln!("Failed to create bucket: {}", e);
                    return;
                }
            },
        };
    }

    
}
