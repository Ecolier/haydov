#![allow(warnings)]

use std::{cmp::min, fs::File, io::Write};
use futures_util::{StreamExt, stream};
use libloading::os;
use minio::s3::{self, builders::UploadPart, segmented_bytes::SegmentedBytes, types::{PartInfo, S3Api}};
use serde::{Deserialize, Serialize};
use url::Url;
use bytes::{BytesMut, BufMut};

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
    osm_bucket_name: String,
}

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

const CONCURRENT_REQUESTS: usize = 1;
const PART_SIZE: usize = 5 * 1024 * 1024; // 5 MiB

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
        osm_bucket_name,
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
    
    // Parse the base URL for the OSM provider
    // This will fail if the URL is invalid, and we handle the error gracefully.
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
    
    let s3_base_url = "http://io:9000/".parse::<minio::s3::http::BaseUrl>().unwrap();
    println!("Trying to connect to MinIO at: `{:?}`", s3_base_url);
    
    let static_provider = minio::s3::creds::StaticProvider::new("haydov", "haydov123", None);
    
    let s3_client = match minio::s3::ClientBuilder::new(s3_base_url.clone())
    .provider(Some(Box::new(static_provider.clone())))
    .build() {
        Ok(client) => client,
        Err(e) => {
            eprintln!("Failed to create S3 client: {}", e);
            return;
        }
    };
    
    // Check if the bucket exists, and create it if it does not.
    // If the bucket already exists, it will return an error that we can handle.
    let osm_bucket = match s3_client.create_bucket(osm_bucket_name.clone()).send().await {
        Ok(_) => {
            println!("Bucket '{}' created successfully.", osm_bucket_name);
            osm_bucket_name
        },
        Err(e) => {
            match &e {
                minio::s3::error::Error::S3Error(e) => {
                    println!("Bucket '{}' already exists.", osm_bucket_name);
                    osm_bucket_name
                },
                _ => {
                    eprintln!("Failed to create bucket: {}", e);
                    return;
                }
            }
        }
    };
    
    let http_client = reqwest::Client::new();
    
    // Send HTTP requests concurrently and collect the responses 
    // using a stream. Each request is spawned as a separate task.
    // The responses are processed to download the content and save it to files.
    // 
    let download = stream::iter(region_urls).for_each_concurrent(CONCURRENT_REQUESTS, |url| {
        
        let http_client = http_client.clone();
        let s3_client = s3_client.clone();
        let osm_bucket = osm_bucket.clone();
        
        async move {
            let response = match http_client.get(url.clone()).send().await {
                Ok(response) => response,
                Err(_) => {
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
                    last_segment
                },
                None => {
                    eprintln!("Failed to parse URL path segments for '{}'", url);
                    return;
                }
            };
            
            let mut downloaded_size: u64 = 0;
            let mut stream = response.bytes_stream();
            let mut buffer = BytesMut::with_capacity(PART_SIZE);
            let mut part_number = 1u16;
            let mut parts: Vec<PartInfo> = vec![];
            
            let upload_id = match s3_client.create_multipart_upload(&osm_bucket, filename).send().await {
                Ok(response) => response.upload_id,
                Err(e) => {
                    eprintln!("Failed to create multipart upload for '{}': {}", filename, e);
                    return;
                }
            };
            
            // Process the stream of bytes, writing them to the file
            // and updating the downloaded size.
            while let Some(item) = stream.next().await {
                
                let chunk = item
                .or(Err(format!("Error while downloading file")))
                .unwrap();
                
                buffer.extend_from_slice(&chunk);
                let chunk_len = chunk.len() as u64;
                
                while buffer.len() >= PART_SIZE {
                    let part_bytes = buffer.split_to(PART_SIZE).freeze();

                    let part_resp = match s3_client.upload_part(&osm_bucket, filename, &upload_id, part_number, SegmentedBytes::from(part_bytes)).send().await {
                        Ok(resp) => resp,
                        Err(e) => {
                            eprintln!("Failed to upload part {}: {}", part_number, e);
                            return;
                        }
                    };

                    println!("Uploaded part {}", part_number);

                    parts.push(PartInfo {
                        number: part_number,
                        size: PART_SIZE as u64,
                        etag: part_resp.etag,
                    });
                    
                    part_number += 1;
                }

                // Upload final (possibly small) part
                if !buffer.is_empty() {
                    let final_part = buffer.clone().freeze();
                    let part_resp = match s3_client.upload_part(&osm_bucket, filename, &upload_id, part_number, SegmentedBytes::from(final_part)).send().await {
                        Ok(resp) => resp,
                        Err(e) => {
                            eprintln!("Failed to upload part {}: {}", part_number, e);
                            return;
                        }
                    };

                    println!("Uploaded final part {}", part_number);

                    parts.push(PartInfo {
                        number: part_number,
                        size: buffer.len() as u64,
                        etag: part_resp.etag,
                    });
                }
                
                let new = min(downloaded_size + chunk_len, total_size);
                downloaded_size = new;
                // println!(
                //     "Downloaded {} bytes of {} bytes ({:.2}%)",
                //     downloaded_size,
                //     total_size,
                //     (downloaded_size as f64 / total_size as f64) * 100.0
                // );
            }
        }
    });
    
    // Wait for all downloads to complete
    download.await;
    
}
