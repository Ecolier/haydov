use std::collections::HashMap;
use thiserror::Error;
use url::{Url};

use crate::region::Region;

#[derive(Debug, Error)]
pub enum DownloadError {
    #[error("Malformed URL error")]
    ParseUrlError(#[from] url::ParseError),

    #[error("Request error")]
    RequestError(#[from] reqwest::Error),
}

fn collect_urls<'a>(
    regions: &'a Vec<Region>,
    base_path: &Url,
    out_list: &mut Vec<Result<(&'a str, Url), DownloadError>>,
) {
    for region in regions {
        match region {
            Region::Node {
                name: _,
                path,
                regions,
            } => match base_path.join(&path) {
                Ok(new_base) => collect_urls(regions, &new_base, out_list),
                Err(e) => out_list.push(Err(DownloadError::ParseUrlError(e))),
            },
            Region::Leaf { name: _, file } => {
                let file_url = match base_path.join(file) {
                    Ok(url) => out_list.push(Ok((file, url))),
                    Err(e) => out_list.push(Err(DownloadError::ParseUrlError(e))),
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
) -> Vec<Result<(&'a str, Url), DownloadError>> {
    let mut download_list = Vec::new();
    collect_urls(regions, base_path, &mut download_list);
    download_list
}
