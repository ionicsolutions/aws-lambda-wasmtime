use std::env;

use serde_json::Value;
use wasmtime::component::{bindgen, Component, Linker, ResourceTable};
use wasmtime::{Engine, Result, Store};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiView};

bindgen!({
    path: "../function/wit/world.wit",
    additional_derives: [
        serde::Deserialize,
        serde::Serialize,
    ],
});

fn main() -> Result<()> {
    let engine = Engine::default();
    let component = Component::from_file(
        &engine,
        "../function/target/wasm32-wasip1/debug/function.wasm",
    )?;

    let mut linker = Linker::<BasicState>::new(&engine);
    wasmtime_wasi::add_to_linker_sync(&mut linker)?;

    let mut store = Store::new(
        &engine,
        BasicState {
            ctx: WasiCtxBuilder::new().build(),
            table: ResourceTable::new(),
        },
    );
    let bindings = LambdaFunction::instantiate(&mut store, &component, &linker)?;
    let lambda = bindings.component_function_lambda();

    let args: Vec<String> = env::args().collect();
    let value: Value =
        serde_json::from_str(&args[1]).expect("Failed to parse the input JSON event");
    let event: exports::component::function::lambda::Event = serde_json::from_value(value)?;

    let response = lambda.call_handler(&mut store, event)?;
    println!("{}", serde_json::to_string(&response)?);

    Ok(())
}

struct BasicState {
    ctx: WasiCtx,
    table: ResourceTable,
}

impl WasiView for BasicState {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}
