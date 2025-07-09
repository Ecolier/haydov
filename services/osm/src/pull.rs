#![allow(warnings)]   

use config;
use anyhow::{Result, Context};
use futures_util::{stream, StreamExt, TryStreamExt};
use log::error;
use minio::s3::{http::BaseUrl, segmented_bytes::SegmentedBytes, types::S3Api};

mod download;
mod storage;
mod region;
mod settings;

use settings::Settings;

use crate::storage::StorageBackend;

// Default values for concurrent requests and chunk size
// These values can be overridden by the configuration file or environment variables.
const STREAM_CONCURRENT_REQUESTS: usize = 4;
const STREAM_CHUNK_SIZE: usize = 5 * 1024 * 1024;

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
    let config = config.try_deserialize::<settings::Settings>()?;

    let storage_base_url = config.storage_base_url.to_string();
    let (storage_client, bucket_name) = storage::init(
        &storage_base_url,
        &config.storage_username,
        &config.storage_password,
        &config.storage_bucket_name,
    ).await?;

    // Use default values for concurrent requests and chunk size if not provided
    // This allows the user to override these values in the configuration file or environment variables.
    let stream_concurrent_requests = config.stream_concurrent_requests.unwrap_or(STREAM_CONCURRENT_REQUESTS);
    let stream_chunk_size = config.stream_chunk_size.unwrap_or(STREAM_CHUNK_SIZE);
    
    let http_client = reqwest::Client::new();
    
    let region_download_list = download::create_download_list(&config.regions, &config.download_base_url);
    
    stream::iter(region_download_list).try_for_each_concurrent(stream_concurrent_requests, |(object, url)| {
        let http_client = http_client.clone();
        let storage_client = storage_client.clone();
        let bucket_name = bucket_name.clone();

        println!("Downloading object: {} for url {}", object, url);

        async move {
            let response = http_client.get(url.clone()).send().await?;
            let content_length = response.content_length();

            println!("Content length for {}: {:?}", object, content_length);
            
            // If the file is too large, we need to buffer it.
            // Assuming stream_multipart is async and returns a Result.
            storage_client.stream_multipart(
                &bucket_name,
                object,
                response.bytes_stream(),
                content_length,
                stream_chunk_size,
            ).await;

            Ok(())
        }
    }).await?;

    Ok(())
}

// If the total size is less than the chunk size, we can skip the buffer logic.
    // This is useful for small files that can be uploaded in one go.
    // This check prevents unnecessary buffering for small files.
    // if (total_size as usize) < chunk_size {
    //     match s3_client
    //     .put_object(
    //         &osm_bucket,
    //         filename,
    //         SegmentedBytes::from(response.bytes().await.unwrap()),
    //     )
    //     .send()
    //     .await
    //     {
    //         Ok(resp) => {
    //             println!(
    //                 "Small file '{}' uploaded successfully: {:?}",
    //                 filename, resp.object
    //             );
    //             return;
    //         }
    //         Err(e) => {
    //             eprintln!("Failed to upload file '{}': {}", filename, e);
    //             return;
    //         }
    //     }
    
    //     s3_client.stream_multipart(bucket, object, stream, total_size, chunk_size).
    
    //     // Process each response as it arrives
    //     storage::Storage::upload_multipart_from_stream(
    //         response.bytes_stream(),
    //         total_size,
    //         chunk_size,
    //     );
    // })
    // .await;