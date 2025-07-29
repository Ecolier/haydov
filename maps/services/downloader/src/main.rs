mod types;

use anyhow::{Context, Result};
use config;
use types::Settings;
use wasmtime_wasi::{
    p2::{add_to_linker_sync, IoView, WasiCtx, WasiCtxBuilder, WasiView}, ResourceTable
};

use wasmtime::component::*;
use wasmtime::{Engine, Store};

bindgen!({
    world: "data-provider",
    path: "examples/data-provider",
});

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
    let Settings {storage, downloader, provider} = config.try_deserialize::<Settings>()?;
    let engine = Engine::default();
    let component = Component::from_file(&engine, &provider.component)
        .context("Failed to load component")?;
    let mut linker = Linker::new(&engine);
    add_to_linker_sync(&mut linker)?;
    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()
        .build();
    let host = DataProviderState {
        ctx: wasi,
        table: ResourceTable::new(),
    };
    let mut store = Store::new(&engine, host);
    let bindings = DataProvider::instantiate(&mut store, &component, &linker)?;
    let config_bytes = serde_json::to_vec(&provider.schema)
        .context("Failed to serialize schema to JSON bytes")?;
    let result = bindings.call_parse_urls(store, &config_bytes);
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
