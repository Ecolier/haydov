#![allow(warnings)]

use std::sync::Arc;

use anyhow::{Context, Result};
use aws_config::Region;
use aws_sdk_s3::{
    config::Credentials,
    operation::upload_part::UploadPart,
    primitives::ByteStream,
    types::{CompletedMultipartUpload, CompletedPart, builders::CompletedPartBuilder},
};
use bytes::BytesMut;
use config;
use futures_util::{StreamExt, TryStreamExt, stream};
use log::{error, info};

mod download;
mod errors;
mod region;
mod settings;

use settings::Settings;

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

    // let bucket = match storage_client
    //     .create_bucket()
    //     .bucket(&config.bucket_name)
    //     .send()
    //     .await {
    //     Ok(_) => {
    //         info!("Bucket {} created successfully.", config.bucket_name);
    //         &config.bucket_name
    //     }
    //     Err(e) => {
    //         error!("Failed to create bucket {}: {}", &config.bucket_name, e);
    //         return Err(e.into());
    //     }
    // };

    // Use default values for concurrent requests and chunk size if not provided
    // This allows the user to override these values in the configuration file or environment variables.
    let stream_concurrent_requests = config.concurrent_requests.unwrap_or(CONCURRENT_REQUESTS);

    let region_download_list =
        download::create_download_list(&config.regions, &config.download_base_url);

    let http_client = Arc::new(reqwest::Client::new());

    stream::iter(region_download_list)
        .try_for_each_concurrent(stream_concurrent_requests, |(object, url)| {
            let http_client = http_client.clone();
            let storage_client = storage_client.clone();
            let value = config.clone();
            async move {
                let response = http_client.get(url.clone()).send().await?;
                let content_length = response.content_length();

                info!("Content length for {}: {:?}", object, content_length);

                // If the file is too large, we need to buffer it.
                // Assuming stream_multipart is async and returns a Result.
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

                        println!(
                            "Uploading part {} for object {} with size {} bytes",
                            parts_count,
                            object,
                            part_bytes.len()
                        );

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
