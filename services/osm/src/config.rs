use serde::{Deserialize, Serialize};
use url::Url;

use crate::osm::Region;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub regions: Vec<Region>,
    pub storage_base_url: Url,
    pub storage_bucket_name: String,
    pub storage_username: String,
    pub storage_password: String,
    pub download_base_url: Url,
    pub stream_concurrent_requests: Option<usize>,
    pub stream_chunk_size: Option<usize>,
}