#![allow(unused)]

mod types;
mod script_runner;

use anyhow::{Context, Result};
use aws_config::Region;
use aws_sdk_s3::config::Credentials;
use config;
use maps_utils::S3ClientExt;
use std::sync::Arc;
use types::Settings;
use script_runner::RhaiRunner;

#[tokio::main]
async fn main() -> Result<()> {
    let config_path = std::env::var("CONFIG_PATH")
        .unwrap_or_else(|_| "/app/config.yaml".to_string());

    let config = config::Config::builder()
        .add_source(config::File::with_name(&config_path))
        .add_source(config::Environment::default())
        .build()
        .context("Failed to build configuration")?;

    let config = Arc::new(config.try_deserialize::<Settings>()?);

    // Initialize Rhai script runner
    let mut script_runner = RhaiRunner::new();
    let urls = script_runner
        .generate_urls(&config.script, &config.schema);

    println!("Generated URLs: {:?}", urls);

    // let storage_client = Arc::new(aws_sdk_s3::Client::from_conf(
    //     aws_sdk_s3::config::Builder::new()
    //         .endpoint_url(&config.storage.base_url.to_string())
    //         .credentials_provider(Credentials::new(
    //             &config.storage.username,
    //             &config.storage.password,
    //             None,
    //             None,
    //             "environment",
    //         ))
    //         .region(Region::new(config.storage.region.clone()))
    //         .behavior_version_latest()
    //         .force_path_style(true)
    //         .build(),
    // ));

    Ok(())
}
