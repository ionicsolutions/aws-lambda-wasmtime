use std::env;

use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};
use serde_json::Value;
use wasmtime::component::{Component, Linker};
use wasmtime::Engine;
use wasmtime_wasi::add_to_linker_sync;

mod wasm_handler;
use wasm_handler::{function_handler, BasicState, LambdaFunctionPre};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    let engine = Engine::default();

    let mut linker = Linker::<BasicState>::new(&engine);
    add_to_linker_sync(&mut linker)?;

    let lambda_task_root = env::var("LAMBDA_TASK_ROOT").unwrap_or_else(|_| ".".to_string());
    let component_file = env::var("_HANDLER")?;
    let component_path = format!("{}/{}", lambda_task_root, component_file);
    let component = Component::from_file(&engine, component_path)?;

    let instance_pre = linker.instantiate_pre(&component)?;
    let pre_instantiated_component = LambdaFunctionPre::new(instance_pre)?;

    let shared_engine = &engine;
    let shared_pre_instantiated_component = &pre_instantiated_component;
    run(service_fn(move |event: LambdaEvent<Value>| async move {
        function_handler(&shared_engine, &shared_pre_instantiated_component, event).await
    }))
    .await
}
