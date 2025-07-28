use anyhow::{Context, Result};
use aws_config::Region;
use aws_sdk_s3::config::Credentials;
use config;
use maps_dispatcher::{Dispatcher, DispatcherConfig};
use maps_dispatcher::providers::osm::OsmProvider;
use std::sync::Arc;

mod settings;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().context("Failed to load .env file")?;
    dotenvy::from_filename(".env.secret").context("Failed to load .env.secret file")?;

    let config = config::Config::builder()
        .add_source(config::File::with_name("config.json"))
        .add_source(config::Environment::default())
        .build()
        .context("Failed to build configuration")?;

    let config = Arc::new(config.try_deserialize::<settings::Settings>()?);

    let storage_client = Arc::new(aws_sdk_s3::Client::from_conf(
        aws_sdk_s3::config::Builder::new()
            .endpoint_url(&config.storage_base_url.to_string())
            .credentials_provider(Credentials::new(
                &config.storage_username,
                &config.storage_password,
                None,
                None,
                "environment",
            ))
            .region(Region::new(config.storage_region.clone()))
            .behavior_version_latest()
            .force_path_style(true)
            .build(),
    ));

    // Create dispatcher config
    let dispatcher_config = DispatcherConfig {
        bucket_name: config.raw_bucket_name.clone(),
        default_concurrent_requests: config.concurrent_requests.unwrap_or(4),
        default_chunk_size: config.chunk_size.unwrap_or(5 * 1024 * 1024),
    };

    // Create dispatcher and provider
    let dispatcher = Dispatcher::new(storage_client, dispatcher_config);
    let osm_provider = OsmProvider::new(
        config.regions.clone(),
        config.osm_download_base_url.clone(),
    );

    // Execute dispatch
    dispatcher
        .dispatch(osm_provider, config.concurrent_requests)
        .await
        .context("Failed to dispatch OSM downloads")?;

    Ok(())
}