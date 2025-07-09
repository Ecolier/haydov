use std::collections::HashMap;
use aws_sdk_s3::operation::create_multipart_upload::{CreateMultipartUpload, CreateMultipartUploadError};
use thiserror::Error;
use url::{Url};

use crate::{errors::Error, region::Region};

fn collect_urls<'a>(
    regions: &'a Vec<Region>,
    base_path: &Url,
    out_list: &mut Vec<Result<(&'a str, Url), Error>>,
) {
    for region in regions {
        match region {
            Region::Node {
                name: _,
                path,
                regions,
            } => match base_path.join(&path) {
                Ok(new_base) => collect_urls(regions, &new_base, out_list),
                Err(e) => out_list.push(Err(Error::ParseUrlError(e))),
            },
            Region::Leaf { name: _, file } => {
                let file_url = match base_path.join(file) {
                    Ok(url) => out_list.push(Ok((file, url))),
                    Err(e) => out_list.push(Err(Error::ParseUrlError(e))),
                };
            }
        }
    }
}

// Collects URLs from the region structure, recursively traversing nodes and leaves.
// For nodes, it appends the path to the base URL and collects URLs from child regions.
// For leaves, it appends the file name to the base URL.
// Returns a vector of URLs as strings.
pub fn create_download_list<'a>(
    regions: &'a Vec<Region>,
    base_path: &Url,
) -> Vec<Result<(&'a str, Url), Error>> {
    let mut download_list = Vec::new();
    collect_urls(regions, base_path, &mut download_list);
    download_list
}
