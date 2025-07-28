use anyhow::Result;
use async_trait::async_trait;
use std::time::SystemTime;
use url::Url;

#[derive(Debug, Clone)]
pub struct DownloadItem {
    pub name: String,
    pub url: Url,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug)]
pub struct BatchInfo {
    pub batch_id: String,
    pub date_prefix: String,
    pub timestamp: SystemTime,
}

#[derive(Debug, Clone)]
pub struct DispatcherConfig {
    pub bucket_name: String,
    pub default_concurrent_requests: usize,
    pub default_chunk_size: usize,
}

#[async_trait]
pub trait DownloadProvider: Send + Sync {
    async fn create_download_list(&self) -> Result<Vec<DownloadItem>>;
}