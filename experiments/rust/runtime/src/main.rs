use std::env;

use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};
use serde_json::Value;
use wasi_common::sync::add_to_linker;
use wasi_common::WasiCtx;
use wasmtime::{Engine, Linker, Module};
mod wasm_handler;
use wasm_handler::function_handler;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    let engine = Engine::default();

    let mut linker = Linker::new(&engine);
    add_to_linker(&mut linker, |s: &mut WasiCtx| s)?;

    let module_path = env::var("WASM_MODULE_PATH")?;
    let module = Module::from_file(&engine, module_path)?;

    let shared_engine = &engine;
    let shared_linker = &linker;
    let shared_module = &module;
    run(service_fn(move |event: LambdaEvent<Value>| async move {
        function_handler(shared_engine, shared_linker, shared_module, event).await
    }))
    .await
}
