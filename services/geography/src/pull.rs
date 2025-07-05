use std::{fs::File, io::Write, cmp::min};
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

fn collect_urls(region: &Region, base_path: String) -> Vec<String> {
    match region {
        Region::Node {
            name: _,
            path,
            regions,
        } => {
            let path = format!("{}/{}", base_path, path);
            regions
            .iter()
            .flat_map(|child| collect_urls(child, path.clone()))
            .collect()
        }
        Region::Leaf { name: _, file } => {
            let path = format!("{}/{}", base_path, file);
            vec![path]
        }
    }
}

const PARALLEL_REQUESTS: usize = 2;

#[tokio::main]
async fn main() {
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
    
    let region_urls: Vec<String> = regions
    .iter()
    .flat_map(|region| collect_urls(region, osm_provider_base_url.clone()))
    .collect();
    
    let http_client = reqwest::Client::new();
    
    let bodies = stream::iter(region_urls)
    .map(|url| {
        let http_client = http_client.clone();
        tokio::spawn(async move {
            let resp = http_client.get(&url).send().await.unwrap();
            
            let total_size = resp
            .content_length().unwrap();
            
            print!("Downloading {} bytes from '{}'\n", total_size, &url);
            
            let filename = Url::parse(&url);
            let binding = filename.unwrap();
            let filename = binding.path_segments().unwrap().last().unwrap();
            
            let mut file = File::create(&filename)
            .or(Err(format!(
                "Failed to create file '{}'",
                "./downloaded.osm.pbf"
            )))
            .unwrap();
            
            let mut downloaded: u64 = 0;
            let mut stream = resp.bytes_stream();
            while let Some(item) = stream.next().await {
                let chunk = item
                .or(Err(format!("Error while downloading file")))
                .unwrap();
                let _ = file
                .write_all(&chunk)
                .or(Err(format!("Error while writing to file")));
                let new = min(downloaded + (chunk.len() as u64), total_size);
                downloaded = new;
                println!(
                    "Downloaded {} bytes of {} bytes ({:.2}%)",
                    downloaded,
                    total_size,
                    (downloaded as f64 / total_size as f64) * 100.0
                );
            }
            stream
        })
    })
    .buffer_unordered(PARALLEL_REQUESTS);
    
    bodies
    .for_each(|b| async {
        match b {
            Ok(_) => println!("Got bytes"),
            Err(e) => eprintln!("Got an error: {}", e),
        }
    })
    .await;
}
