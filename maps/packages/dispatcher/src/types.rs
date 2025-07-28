use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub dispatcher: DispatcherConfig,
    pub schema: Value,
    pub script: String,
    pub storage: StorageConfig,
}

#[derive(Debug, Deserialize)]
pub struct DispatcherConfig {
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