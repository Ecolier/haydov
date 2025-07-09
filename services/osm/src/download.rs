use url::{Url};

use crate::{errors::Error, region::Region};

/// Collects URLs from the given regions and constructs full URLs based on the base path.
/// This function recursively traverses the regions, appending paths to the base URL
/// and collecting leaf URLs.
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
                match base_path.join(file) {
                    Ok(url) => out_list.push(Ok((file, url))),
                    Err(e) => out_list.push(Err(Error::ParseUrlError(e))),
                };
            }
        }
    }
}

pub fn create_download_list<'a>(
    regions: &'a Vec<Region>,
    base_path: &Url,
) -> Vec<Result<(&'a str, Url), Error>> {
    let mut download_list = Vec::new();
    collect_urls(regions, base_path, &mut download_list);
    download_list
}
