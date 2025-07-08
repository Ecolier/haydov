use std::collections::HashMap;
use url::{Url, ParseError};

mod region;

// Collects URLs from the region structure, recursively traversing nodes and leaves.
// For nodes, it appends the path to the base URL and collects URLs from child regions.
// For leaves, it appends the file name to the base URL.
// Returns a vector of URLs as strings.
pub fn create_download_list<'a>(regions: &'a Vec<Region>, base_path: &Url) -> Result<HashMap<&'a str, Url>, ParseError> {
    let mut download_list: HashMap<&'a str, Url> = HashMap::new();
    for region in regions {
        match region {
            region::Region::Node {
                name: _,
                path,
                regions,
            } => {
                let new_base = base_path.join(&path)?;
                download_list.extend(create_download_list(regions, &new_base)?);
            }
            region::Region::Leaf { name: _, file } => {
                let file_url = base_path.join(file)?;
                download_list.insert(file, file_url);
            }
        }
    }
    Ok(download_list)
}