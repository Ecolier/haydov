use rhai::{Engine, Dynamic, AST};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrlItem {
    pub name: String,
    pub url: String,
    pub metadata: Option<Value>,
}

pub struct RhaiRunner {
    engine: Engine,
}

impl RhaiRunner {
    pub fn new() -> Self {
        let mut engine = Engine::new();

        engine.set_max_expr_depths(1000, 1000);  // Increase expression depth
        engine.set_max_call_levels(1000);        // Increase function call depth
        engine.set_max_operations(1_000_000);    // Increase operation count
        engine.set_max_string_size(1_000_000);   // Increase string size limit
        engine.set_max_array_size(10_000);       // Increase array size limit
        engine.set_max_map_size(10_000);         // Increase map size limit
        
        // Register helper functions for scripts
        engine.register_fn("log", |msg: &str| println!("Script: {}", msg));
        engine.register_fn("debug", |msg: &str| eprintln!("Debug: {}", msg));
        
        // Register type_of function for scripts
        engine.register_fn("type_of", |value: &Dynamic| -> String {
            value.type_name().to_string()
        });
        
        Self { engine }
    }

    pub fn generate_urls<P: AsRef<Path>>(
        &mut self,
        script_path: P,
        schema: &Value,
    ) -> Result<Vec<UrlItem>> {
        let script_path = script_path.as_ref();
        
        // Read and compile the script
        let script_content = std::fs::read_to_string(script_path)
            .with_context(|| format!("Failed to read Rhai script: {}", script_path.display()))?;

        let ast = self.engine.compile(&script_content)
            .context("Failed to compile Rhai script")?;

        let schema_dynamic = rhai::serde::to_dynamic(schema)
            .map_err(|e| anyhow::anyhow!("Failed to convert schema to Dynamic: {}", e))?;

        // Set schema as a global variable using Scope
        let mut scope = rhai::Scope::new();
        scope.push("schema", schema_dynamic);

        println!("scope: {:#?}", scope);
 
        // Execute the script
        let result: Dynamic = self.engine.eval_ast_with_scope(&mut scope, &ast)
            .map_err(|e| anyhow::anyhow!("Failed to execute Rhai script: {}", e))?;

        // Convert result directly using serde
        let urls: Vec<UrlItem> = rhai::serde::from_dynamic(&result)
            .map_err(|e| anyhow::anyhow!("Failed to convert script result to Vec<UrlItem>: {}", e))?;

        Ok(urls)
    }
}