use anyhow::Result;
use aws_sdk_s3::Client;
use chrono::{DateTime, Utc};
use futures::Stream;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use url::Url;

pub mod config;
pub mod storage;
pub mod types;

use crate::types::*;

pub struct Dispatcher {
    storage_client: Arc<Client>,
    config: DispatcherConfig,
}

impl Dispatcher {
    pub fn new(storage_client: Arc<Client>, config: DispatcherConfig) -> Self {
        Self {
            storage_client,
            config,
        }
    }

    pub async fn dispatch<P, S>(
        &self,
        provider: P,
        concurrent_requests: Option<usize>,
    ) -> Result<()>
    where
        P: DownloadProvider,
        S: Stream<Item = Result<bytes::Bytes, Box<dyn std::error::Error + Send + Sync>>> + Send + Unpin,
    {
        let concurrent_requests = concurrent_requests.unwrap_or(self.config.default_concurrent_requests);
        
        // Get download list from provider
        let download_list = provider.create_download_list().await?;
        
        // Create batch metadata
        let batch_info = self.create_batch_info();
        
        // Execute downloads and uploads
        self.process_downloads(download_list, batch_info, concurrent_requests).await?;
        
        // Mark batch as ready
        self.mark_batch_ready(&batch_info).await?;
        
        Ok(())
    }

    fn create_batch_info(&self) -> BatchInfo {
        let now = SystemTime::now();
        let duration = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
        let timestamp_millis = duration.as_millis();
        let datetime: DateTime<Utc> = now.into();
        
        BatchInfo {
            batch_id: timestamp_millis.to_string(),
            date_prefix: datetime.format("%Y/%m/%d").to_string(),
            timestamp: now,
        }
    }

    async fn process_downloads(
        &self,
        download_list: Vec<DownloadItem>,
        batch_info: BatchInfo,
        concurrent_requests: usize,
    ) -> Result<()> {
        // Implementation using your existing request::concurrent logic
        // but adapted to work with the generic DownloadItem
        todo!("Implement download processing")
    }

    async fn mark_batch_ready(&self, batch_info: &BatchInfo) -> Result<()> {
        let key = format!("batches/{}/{}_READY", batch_info.date_prefix, batch_info.batch_id);
        
        self.storage_client
            .put_object()
            .bucket(&self.config.bucket_name)
            .key(key)
            .send()
            .await?;
        
        Ok(())
    }
}