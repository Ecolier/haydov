use serde::{Deserialize, Serialize};
use url::Url;

use crate::osm::Region;

#[derive(Serialize, Deserialize, Debug)]
pub struct RawConfig {
    regions: Vec<Region>,
    storage_base_url: String,
    storage_bucket_name: String,
    storage_username: String,
    storage_password: String,
    download_base_url: String,
    stream_concurrent_requests: Option<usize>,
    stream_chunk_size: Option<usize>,
}

#[derive(Debug)]
pub struct Config {
    regions: Vec<Region>,
    storage_base_url: Url,
    storage_bucket_name: String,
    storage_username: String,
    storage_password: String,
    download_base_url: Url,
    stream_concurrent_requests: Option<usize>,
    stream_chunk_size: Option<usize>,
}

impl TryFrom<RawConfig> for Config {
    type Error = String;
    
    fn try_from(raw: RawConfig) -> Result<Self, Self::Error> {
        
        if raw.regions.is_empty() {
            return Err("At least one region must be specified.".into());
        }
        
        let storage_base_url = Url::parse(&raw.storage_base_url)
        .map_err(|e| format!("Invalid storage_base_url: {}", e))?;
        
        let download_base_url = Url::parse(&raw.download_base_url)
        .map_err(|e| format!("Invalid download_base_url: {}", e))?;
        
        if raw.storage_bucket_name.is_empty() {
            return Err("storage_bucket_name must not be empty.".into());
        }
        
        if raw.storage_username.is_empty() {
            return Err("storage_username must not be empty.".into());
        }
        
        if raw.storage_password.is_empty() {
            return Err("storage_password must not be empty.".into());
        }
        
        Ok(Config {
            download_base_url,
            storage_base_url,
            regions: raw.regions,
            storage_bucket_name: raw.storage_bucket_name,
            storage_username: raw.storage_username,
            storage_password: raw.storage_password,
            stream_concurrent_requests: raw.stream_concurrent_requests,
            stream_chunk_size: raw.stream_chunk_size,
        })
    }
}