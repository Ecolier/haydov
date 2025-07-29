wit_bindgen::generate!({
    world: "dispatcher-world",
});

use exports::maps::plugin::dispatcher::{Guest, Schema};

struct DispatcherComponent;

impl Guest for DispatcherComponent {
    fn parse_urls(schema: Schema) -> Vec<String> {
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
