use serde::{Deserialize, Serialize};
use url::Url;

use crate::region::Region;

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub concurrent_requests: Option<usize>,
    pub chunk_size: Option<usize>,
    pub storage_base_url: Url,
    pub storage_region: String,
    pub storage_username: String,
    pub storage_password: String,
    pub bucket_name: String,
}