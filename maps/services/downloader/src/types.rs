use serde::{Deserialize};
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub downloader: DownloaderConfig,
    pub provider: ProviderConfig,
    pub storage: StorageConfig,
}

#[derive(Debug, Deserialize)]
pub struct DownloaderConfig {
    pub concurrent_requests: Option<usize>,
    pub chunk_size: Option<usize>,
    pub bucket_name: String,
}

#[derive(Debug, Deserialize)]
pub struct StorageConfig {
    pub base_url: String,
    pub region: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct ProviderConfig {
    pub component: String,
    pub schema: Value,
}