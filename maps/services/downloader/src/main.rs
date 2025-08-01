mod types;

use anyhow::{Context, Result};
use config;
use types::Settings;
use wasmtime_wasi::{
    ResourceTable,
    p2::{IoView, WasiCtx, WasiCtxBuilder, WasiView, add_to_linker_sync},
};

use wasmtime::component::*;
use wasmtime::{Engine, Store};

struct DataProviderState {
    ctx: WasiCtx,
    table: ResourceTable,
}

impl IoView for DataProviderState {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}
impl WasiView for DataProviderState {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
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

    let Settings {
        storage,
        downloader,
        provider,
    } = config.try_deserialize::<Settings>()?;

    // Get the directory containing the config file
    let config_dir = std::path::Path::new(&config_path)
        .parent()
        .unwrap_or_else(|| std::path::Path::new("."));

     // Resolve component path relative to config directory
    let component_path = if std::path::Path::new(&provider.component).is_absolute() {
        // If already absolute, use as-is
        provider.component.clone()
    } else {
        // Make it relative to config directory
        config_dir.join(&provider.component)
            .to_string_lossy()
            .to_string()
    };

    if !std::path::Path::new(&component_path).exists() {
        return Err(anyhow::anyhow!("Component file does not exist"));
    }

    let engine = Engine::default();
    
    let component =
        Component::from_file(&engine, &component_path).context("Failed to load component")?;
        
    let mut linker = Linker::new(&engine);
    add_to_linker_sync(&mut linker)?;

    let wasi = WasiCtxBuilder::new().inherit_stdio().inherit_args().build();
    let host = DataProviderState {
        ctx: wasi,
        table: ResourceTable::new(),
    };
    let mut store = Store::new(&engine, host);

    let config_bytes =
        serde_json::to_vec(&provider.schema).context("Failed to serialize schema to JSON bytes")?;

    let instance = linker.instantiate(&mut store, &component)?;

    let parse_urls = instance
        .get_func(&mut store, "parse-urls")
        .context("Failed to get `parse-urls` function")?
        .typed::<(Vec<u8>,), (Vec<String>,)>(&mut store)?;

    let (results,) = parse_urls.call(&mut store, (config_bytes,))?;
    for url in results {
        println!("Parsed URL: {}", url);
    }

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
