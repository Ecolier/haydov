use serde::{Deserialize, Serialize};
use url::Url;

use crate::region::Region;

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub regions: Vec<Region>,
    pub osm_download_base_url: Url,
    pub wof_download_base_url: Url,
    pub concurrent_requests: Option<usize>,
    pub bucket_name: String,
    pub storage_access_key_id: String,
    pub storage_secret_access_key: String,
    pub aws_region: String,
    pub aws_default_region: String,
    pub aws_s3_endpoint: String,
}