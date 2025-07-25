use serde::{Deserialize, Serialize};
use url::Url;

use crate::region::Region;

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub regions: Vec<Region>,
    pub osm_download_base_url: Url,
    pub wof_download_base_url: Url,
    pub concurrent_requests: Option<usize>,
    pub geography_raw_bucket_name: String,
    pub geography_storage_base_url: Url,
    pub geography_storage_region: String,
    pub geography_storage_username: String,
    pub geography_storage_password: String,
}