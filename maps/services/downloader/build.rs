use wasmtime::component::bindgen;

fn main() {
  bindgen!({
    world: "data-provider",
    path: "examples/data-provider",
  });
}