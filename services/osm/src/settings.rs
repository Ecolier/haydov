use serde::{Deserialize, Serialize};
use url::Url;

use crate::region::Region;

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub regions: Vec<Region>,
    pub download_base_url: Url,
    pub concurrent_requests: Option<usize>,
    pub bucket_name: String,
    pub aws_access_key_id: String,
    pub aws_secret_access_key: String,
    pub aws_region: String,
    pub aws_default_region: String,
    pub aws_s3_endpoint: String,
}