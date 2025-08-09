wit_bindgen::generate!(in "./");

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct GeofabrikSchema {
    base_url: String,
    filename_template: String,
    regions: Vec<String>,
}

struct DispatcherComponent;

impl Guest for DispatcherComponent {
    fn parse_urls(schema: Vec<u8>) -> Vec<String> {
        let schema: GeofabrikSchema =
            serde_json::from_slice(&schema).expect("Failed to parse configuration as JSON or YAML");
        let mut urls = Vec::new();
        for region in schema.regions {
            let url = schema.filename_template.replace("{region}", &region);
            let full_url = format!("{}/{}", schema.base_url.trim_end_matches('/'), url);
            urls.push(full_url);
        }
        urls
    }
}

export!(DispatcherComponent);
