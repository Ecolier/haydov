#![allow(unused)]

mod types;

use anyhow::{Context, Result};
use aws_config::Region;
use aws_sdk_s3::config::Credentials;
use config;
use maps_utils::S3ClientExt;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use types::Settings;
use wasmtime_wasi::{
    p2::{add_to_linker_sync, IoView, WasiCtx, WasiCtxBuilder, WasiView}, ResourceTable
};

use wasmtime::component::*;
use wasmtime::{Engine, Store};

use crate::exports::maps::plugin::dispatcher::{Schema};

bindgen!({
    world: "dispatcher-world",
    path: "examples/geofabrik-plugin/wit",
});

struct DispatcherState {
    ctx: WasiCtx,
    table: ResourceTable,
}

impl IoView for DispatcherState {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}
impl WasiView for DispatcherState {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}

// Create a serde-compatible version of Schema
#[derive(Deserialize, Serialize)]
struct JsonSchema {
    #[serde(alias = "base-url")]
    base_url: String,
    #[serde(alias = "filename-template")]
    filename_template: String,
    regions: Vec<String>,
}

impl From<JsonSchema> for Schema {
    fn from(json_schema: JsonSchema) -> Self {
        Schema {
            base_url: json_schema.base_url,
            filename_template: json_schema.filename_template,
            regions: json_schema.regions,
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let config_path =
        std::env::var("CONFIG_PATH").unwrap_or_else(|_| "/app/config.yaml".to_string());

    let config = config::Config::builder()
        .add_source(config::File::with_name(&config_path))
        .add_source(config::Environment::default())
        .build()
        .context("Failed to build configuration")?;

    let config = Arc::new(config.try_deserialize::<Settings>()?);

    let engine = Engine::default();
    let component = Component::from_file(&engine, &config.component)
        .context("Failed to load component")?;

    let mut linker = Linker::new(&engine);
    add_to_linker_sync(&mut linker)?;

    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()
        .build();

    let host = DispatcherState {
        ctx: wasi,
        table: ResourceTable::new(),
    };

    let mut store = Store::new(&engine, host);

    let bindings = DispatcherWorld::instantiate(&mut store, &component, &linker)?;
    
    // Deserialize config.schema (serde_json::Value) into JsonSchema, then convert to Schema
    let json_schema: JsonSchema = serde_json::from_value(config.schema.clone())
        .context("Failed to deserialize schema")?;

    let schema: Schema = json_schema.into();
    let result = bindings.maps_plugin_dispatcher().call_parse_urls(store, &schema)?;

    println!("Generated URLs: {:?}", result);

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
