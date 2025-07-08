use serde::{Deserialize, Serialize};
use log::{error};
use url::Url;

mod osm;

use crate::osm::{config::RawConfig, Config};

// Default values for concurrent requests and chunk size
// These values can be overridden by the configuration file or environment variables.
const STREAM_CONCURRENT_REQUESTS: usize = 4;
const STREAM_CHUNK_SIZE: usize = 5 * 1024 * 1024;

#[tokio::main]
async fn main() {
    
    // Load configuration from config.json and environment variables
    let config = match config::Config::builder()
    .add_source(config::File::with_name("config.json"))
    .add_source(config::Environment::default())
    .add_source(config::Environment::with_prefix("OSM"))
    .build()
    {
        Ok(config) => config,
        Err(e) => {
            error!("Failed to load configuration: {}", e);
            return;
        }
    };
    
    // Deserialize the configuration into the Settings struct
    // This will fail if the structure does not match the expected format
    // or if required fields are missing.
    let config = match config.try_deserialize::<RawConfig>() {
        Ok(config) => config,
        Err(e) => {
            error!("Failed to deserialize configuration: {}", e);
            return;
        }
    };

    let config = match Config::try_from(config) {
    Ok(s) => s,
    Err(e) => {
        error!("Invalid configuration: {}", e);
        return;
    }
};

    // Use default values for concurrent requests and chunk size if not provided
    // This allows the user to override these values in the configuration file or environment variables.
    let stream_concurrent_requests = config.stream_concurrent_requests.unwrap_or(STREAM_CONCURRENT_REQUESTS);
    let stream_chunk_size = config.stream_chunk_size.unwrap_or(STREAM_CHUNK_SIZE);

    if config.regions.is_empty() {
        error!("No regions found in the configuration.");
        return;
    }
    
    // Parse the base URL for the OSM provider
    // This will fail if the URL is invalid, and we handle the error gracefully.
    let osm_provider_base_url = match Url::parse(&settings.download_base_url) {
        Ok(url) => url,
        Err(e) => {
            error!("Invalid base URL '{}': {}", settings.download_base_url, e);
            return;
        }
    };
    
    let s3_static_provider = minio::s3::creds::StaticProvider::new("haydov", "haydov123", None);
    let s3_base_url = "http://io:9000/"
    .parse::<minio::s3::http::BaseUrl>()
    .unwrap();
    let s3_client = match minio::s3::ClientBuilder::new(s3_base_url.clone())
    .provider(Some(Box::new(s3_static_provider.clone())))
    .build()
    {
        Ok(client) => client,
        Err(e) => {
            eprintln!("Failed to create S3 client: {}", e);
            return;
        }
    };
    
    let storage = osm::storage::stream(&self, stream, total_size, chunk_size)
    
    let http_client = reqwest::Client::new();
    osm::Download::new(&osm_provider_base_url, &regions)
    .stream(&http_client, concurrent_requests, |stream, content_length| {
        
        if content_length < chunk_size as u64 {
            eprintln!("Content length {} is less than chunk size {}", content_length, chunk_size);
            return;
        }
        
        // Process each response as it arrives
        storage::Storage::upload_multipart_from_stream(
            response.bytes_stream(),
            total_size,
            chunk_size,
        );
    })
    .await;
    
}
