use futures_util::{Stream, StreamExt, stream};

use crate::osm::Region;

// Collects URLs from the region structure, recursively traversing nodes and leaves.
// For nodes, it appends the path to the base URL and collects URLs from child regions.
// For leaves, it appends the file name to the base URL.
// Returns a vector of URLs as strings.
fn collect_urls(region: &Region, base_path: &url::Url) -> Vec<url::Url> {
    match region {
        Region::Node {
            name: _,
            path,
            regions,
        } => {
            let path = base_path.join(path).unwrap();
            regions
                .iter()
                .flat_map(|child| collect_urls(child, &path))
                .collect()
        }
        Region::Leaf { name: _, file } => {
            let path = base_path.join(file).unwrap();
            vec![path]
        }
    }
}

pub struct Download<'a> {
    base_url: &'a url::Url,
    regions: &'a Vec<Region>,
}

impl<'a> Download<'a> {
    pub fn new(base_url: &'a url::Url, regions: &'a Vec<Region>) -> Self {
        Download { base_url, regions }
    }

    pub fn stream(
        &self,
        http_client: &reqwest::Client,
        concurrent_requests: usize,
        closure: impl Fn(Box<dyn Stream<Item = reqwest::Result<bytes::Bytes>>>, u64),
    ) {
        // Collect URLs from the regions
        let region_urls: Vec<url::Url> = self
            .regions
            .iter()
            .flat_map(|region| collect_urls(region, &self.base_url))
            .collect();

        // Send HTTP requests concurrently and collect the responses
        // using a stream. Each request is spawned as a separate task.
        let download = stream::iter(region_urls).for_each_concurrent(concurrent_requests, |url| {
            // Clone the HTTP client and S3 client for each task to avoid ownership issues.
            // This allows each task to use its own instance of the client without conflicts.
            let http_client = http_client.clone();

            // Spawn a new asynchronous task to handle the download and upload process.
            async move {
                let response = match http_client.get(url.clone()).send().await {
                    Ok(response) => response,
                    Err(_) => {
                        eprintln!("Failed to get response for '{}'", url);
                        return;
                    }
                };

                closure(
                    Box::new(response.bytes_stream().boxed()),
                    response.content_length().unwrap_or(0),
                );

                // Extract the filename from the URL path segments
                let filename = match url.path_segments() {
                    Some(segments) => {
                        let last_segment = segments.last().unwrap_or("default.osm.pbf");
                        last_segment
                    }
                    None => {
                        eprintln!("Failed to parse URL path segments for '{}'", url);
                        return;
                    }
                };

                // If the total size is less than the chunk size, we can skip the buffer logic.
                // This is useful for small files that can be uploaded in one go.
                // This check prevents unnecessary buffering for small files.
                if (total_size as usize) < chunk_size {
                    println!(
                        "Small file '{}' detected, uploading directly without buffering",
                        filename
                    );
                    match s3_client
                        .put_object(
                            &osm_bucket,
                            filename,
                            SegmentedBytes::from(response.bytes().await.unwrap()),
                        )
                        .send()
                        .await
                    {
                        Ok(resp) => {
                            println!(
                                "Small file '{}' uploaded successfully: {:?}",
                                filename, resp.object
                            );
                            return;
                        }
                        Err(e) => {
                            eprintln!("Failed to upload file '{}': {}", filename, e);
                            return;
                        }
                    }
                }
            }
        });
    }
}
