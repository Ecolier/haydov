use std::sync::Arc;

use anyhow::Result;
use aws_config::Region;
use aws_sdk_s3::{
    config::Credentials,
    primitives::ByteStream,
    types::{CompletedMultipartUpload, CompletedPart},
};
use bytes::BytesMut;
use config;
use futures_util::{StreamExt, TryStreamExt, stream};

mod download;
mod errors;
mod region;
mod settings;

// Default values for concurrent requests and chunk size
// These values can be overridden by the configuration file or environment variables.
const CONCURRENT_REQUESTS: usize = 4;
const CHUNK_SIZE: usize = 5 * 1024 * 1024;

#[tokio::main]
async fn main() -> Result<()> {

    // Load configuration from config.json and environment variables
    let config = config::Config::builder()
        .add_source(config::File::with_name("config.json"))
        .add_source(config::Environment::default())
        .add_source(config::Environment::with_prefix("OSM"))
        .build()?;

    // Deserialize the configuration into the Settings struct
    // This will fail if the structure does not match the expected format
    // or if required fields are missing.
    let config = Arc::new(config.try_deserialize::<settings::Settings>()?);

    // Create the S3 client and bucket
    // This will use the AWS SDK to create a new S3 bucket if it does not already exist.
    // If the bucket already exists, it will return an error.
    let credentials = Credentials::new(
        &config.aws_access_key_id,
        &config.aws_secret_access_key,
        None,
        None,
        "loaded-from-custom-env",
    );
    let s3_config = aws_sdk_s3::config::Builder::new()
        .endpoint_url(&config.aws_s3_endpoint)
        .credentials_provider(credentials)
        .region(Region::new("eu-central-1"))
        .behavior_version_latest()
        .force_path_style(true) // apply bucketname as path param instead of pre-domain
        .build();

    let storage_client = Arc::new(aws_sdk_s3::Client::from_conf(s3_config));
    
    // Use default values for concurrent requests and chunk size if not provided.
    // This allows the user to override these values in the configuration file or environment variables.
    let stream_concurrent_requests = config.concurrent_requests.unwrap_or(CONCURRENT_REQUESTS);

    // Create a download list from the regions defined in the configuration.
    let region_download_list =
        download::create_download_list(&config.regions, &config.download_base_url);

    let http_client = Arc::new(reqwest::Client::new());

    // Iterate over the download list and perform concurrent downloads.
    // This will download each file in the list concurrently, using the specified number of concurrent requests
    // and chunk size. Each file will be uploaded to the S3 bucket in parts.
    stream::iter(region_download_list)
        .try_for_each_concurrent(stream_concurrent_requests, |(object, url)| {
            let http_client = http_client.clone();
            let storage_client = storage_client.clone();
            let value = config.clone();

            async move {
                let response = http_client.get(url.clone()).send().await?;
                let _content_length = response.content_length();
                
                let multipart_upload = storage_client
                    .create_multipart_upload()
                    .bucket(&value.bucket_name)
                    .key(object)
                    .send()
                    .await?;

                let upload_id = multipart_upload.upload_id().unwrap_or_default();
                let mut buffer = BytesMut::with_capacity(CHUNK_SIZE);
                let mut parts_count = 1i32;
                let mut completed_parts = Vec::new();
                let mut stream = response.bytes_stream();

                while let Some(bytes) = stream.next().await {
                    let bytes = bytes?;
                    buffer.extend_from_slice(&bytes);

                    if buffer.len() >= CHUNK_SIZE {
                        let part_bytes = buffer.split_to(CHUNK_SIZE).freeze();

                        let upload_part_resp = storage_client
                            .upload_part()
                            .bucket(&value.bucket_name)
                            .key(object)
                            .part_number(parts_count)
                            .body(ByteStream::from(part_bytes))
                            .upload_id(upload_id)
                            .send()
                            .await?;

                        completed_parts.push(
                            CompletedPart::builder()
                                .set_part_number(Some(parts_count))
                                .set_e_tag(upload_part_resp.e_tag().map(|s| s.to_string()))
                                .build(),
                        );
                        parts_count += 1;
                    }
                }

                // If there are remaining bytes in the buffer, upload them as the last part.
                // This ensures that any remaining data is uploaded, even if it's less than PART_SIZE.
                if !buffer.is_empty() {
                    let final_part = buffer.freeze();

                    let upload_part_resp = storage_client
                        .upload_part()
                        .bucket(&value.bucket_name)
                        .key(object)
                        .part_number(parts_count)
                        .body(ByteStream::from(final_part))
                        .upload_id(upload_id)
                        .send()
                        .await?;

                    completed_parts.push(
                        CompletedPart::builder()
                            .set_part_number(Some(parts_count))
                            .set_e_tag(upload_part_resp.e_tag().map(|s| s.to_string()))
                            .build(),
                    );
                }

                let completed_upload = CompletedMultipartUpload::builder()
                    .set_parts(Some(completed_parts))
                    .build();

                storage_client
                    .complete_multipart_upload()
                    .bucket(&value.bucket_name)
                    .key(object)
                    .multipart_upload(completed_upload)
                    .upload_id(upload_id)
                    .send()
                    .await?;

                println!("Multipart upload created for object: {}", object);

                Ok(())
            }
        })
        .await?;

    Ok(())
}
