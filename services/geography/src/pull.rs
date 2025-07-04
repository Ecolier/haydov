use std::fs::File;
use futures_util::StreamExt;
use std::io::Write;
use std::cmp::min;

#[tokio::main]
async fn main() {
    let config = config::Config::builder()
        .add_source(config::Environment::default())
        .build()
        .unwrap();
    let osm_provider_base_url = config.get_string("osm_provider_base_url").unwrap();
    let osm_provider_url = format!("{}/europe/france/rhone-alpes-latest.osm.pbf", osm_provider_base_url);
    let response = reqwest::get(&osm_provider_url).await.unwrap();
    let total_size = response
        .content_length()
        .ok_or(format!("Failed to get content length from '{}'", &osm_provider_url)).unwrap();
    let mut file = File::create("./downloaded.osm.pbf").or(Err(format!("Failed to create file '{}'", "./downloaded.osm.pbf"))).unwrap();
    let mut downloaded: u64 = 0;
    let mut stream = response.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.or(Err(format!("Error while downloading file"))).unwrap();
        let _ = file.write_all(&chunk)
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
}