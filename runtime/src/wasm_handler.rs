use lambda_runtime::{Error, LambdaEvent};
use serde_json::Value;
use wasmtime::component::{bindgen, Component, Linker, ResourceTable};
use wasmtime::{Engine, Store};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiView};

bindgen!({
    path: "../experiments/component/function/wit/world.wit",
    additional_derives: [
        serde::Deserialize,
        serde::Serialize,
    ],
});

pub(crate) async fn function_handler(
    engine: &Engine,
    linker: &Linker<BasicState>,
    component: &Component,
    event: LambdaEvent<Value>,
) -> Result<Value, Error> {
    let ctx = WasiCtxBuilder::new()
        .inherit_stdout()
        .inherit_stderr()
        .build();

    let mut store = Store::new(
        &engine,
        BasicState {
            ctx: ctx,
            table: ResourceTable::new(),
        },
    );

    let bindings = LambdaFunction::instantiate(&mut store, &component, &linker)?;
    let lambda = bindings.component_function_lambda();

    let event: exports::component::function::lambda::Event = serde_json::from_value(event.payload)?;

    let response = lambda.call_handler(&mut store, event)?;

    let response_value = serde_json::to_value(response)?;
    Ok(response_value)
}

pub(crate) struct BasicState {
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
