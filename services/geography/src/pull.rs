use std::{cmp::min, fs::File, io::Write};
use futures_util::{StreamExt, stream};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum Region {
    Node {
        name: String,
        path: String,
        regions: Vec<Region>,
    },
    Leaf {
        name: String,
        file: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
struct Settings {
    regions: Vec<Region>,
    osm_provider_base_url: String,
}

/* Collects URLs from the region structure, recursively traversing nodes and leaves.
   For nodes, it appends the path to the base URL and collects URLs from child regions.
   For leaves, it appends the file name to the base URL.
   Returns a vector of URLs as strings.
*/
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

const PARALLEL_REQUESTS: usize = 2;

#[tokio::main]
async fn main() {

    // Load configuration from config.json and environment variables
    let config = match config::Config::builder()
    .add_source(config::File::with_name("config.json"))
    .add_source(config::Environment::default())
    .build()
    {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Failed to load configuration: {}", e);
            return;
        }
    };
    
    // Deserialize the configuration into the Settings struct
    // This will fail if the structure does not match the expected format
    // or if required fields are missing.
    let Settings {
        osm_provider_base_url,
        regions,
    } = match config.try_deserialize::<Settings>() {
        Ok(settings) => settings,
        Err(e) => {
            eprintln!("Failed to deserialize settings: {}", e);
            return;
        }
    };

    if regions.is_empty() {
        eprintln!("No regions found in the configuration.");
        return;
    }

    let osm_provider_base_url = match Url::parse(&osm_provider_base_url) {
        Ok(url) => url,
        Err(e) => {
            eprintln!("Invalid base URL '{}': {}", osm_provider_base_url, e);
            return;
        }
    };

    // Collect URLs from the regions
    let region_urls: Vec<url::Url> = regions
    .iter()
    .flat_map(|region| collect_urls(region, &osm_provider_base_url))
    .collect();
    
    let http_client = reqwest::Client::new();

    /* Send HTTP requests concurrently and collect the responses 
       using a stream. Each request is spawned as a separate task.
       The responses are processed to download the content and save it to files.
    */
    let bodies = stream::iter(region_urls)
    .map(|url| {
        tokio::spawn(async move {
            let response = match http_client.get(url).send().await {
                Ok(response) => response,
                Err(e) => {
                    eprintln!("Failed to get response for '{}'", url);
                    return;
                }
            };

            let total_size = match response.content_length() {
                Some(size) => size,
                None => {
                    eprintln!("Failed to get content length for '{}'", url);
                    return;
                }
            };
            
            println!("Downloading {} bytes from '{}'", total_size, url);

            // Extract the filename from the URL path segments
            let filename = match url.path_segments() {
                Some(segments) => {
                    let last_segment = segments.last().unwrap_or("default.osm.pbf");
                    format!("./{}", last_segment)
                },
                None => {
                    eprintln!("Failed to parse URL path segments for '{}'", url);
                    return;
                }
            };

            // Create a file to write the downloaded content
            // If the file already exists, it will be overwritten.
            let mut file = File::create(&filename)
            .or(Err(format!(
                "Failed to create file '{}'", filename
            )))
            .unwrap();
            
            let mut downloaded_size: u64 = 0;
            let mut stream = response.bytes_stream();
            
            // Process the stream of bytes, writing them to the file
            // and updating the downloaded size.
            while let Some(item) = stream.next().await {
                let chunk = item
                .or(Err(format!("Error while downloading file")))
                .unwrap();
                let _ = file
                .write_all(&chunk)
                .or(Err(format!("Error while writing to file")));
                let new = min(downloaded_size + (chunk.len() as u64), total_size);
                downloaded_size = new;
                println!(
                    "Downloaded {} bytes of {} bytes ({:.2}%)",
                    downloaded_size,
                    total_size,
                    (downloaded_size as f64 / total_size as f64) * 100.0
                );
            }

            stream
        })
    })
    .buffer_unordered(PARALLEL_REQUESTS);
    
    bodies
    .for_each(|b| async {
        match b {
            Ok(_) => {
                println!("Download completed successfully.");
            },
            Err(e) => eprintln!("Got an error: {}", e),
        }
    })
    .await;
}
