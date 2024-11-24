use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};
use serde_json::Value;
use wasmtime::component::{bindgen, Component, Linker, ResourceTable};
use wasmtime::{Engine, Store};
use wasmtime_wasi::{add_to_linker_sync, WasiCtx, WasiCtxBuilder, WasiView};

include!(concat!(env!("OUT_DIR"), "/bindgen_macro.rs"));

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    let component_path = {
        let lambda_task_root =
            std::env::var("LAMBDA_TASK_ROOT").unwrap_or_else(|_| ".".to_string());
        let component_file = std::env::var("_HANDLER").expect(
            "env variable `_HANDLER` should be set to the relative path to the component file",
        );
        format!("{}/{}", lambda_task_root, component_file)
    };

    let engine = Engine::default();
    let mut linker = Linker::<BasicState>::new(&engine);
    add_to_linker_sync(&mut linker)?;

    let component = Component::from_file(&engine, component_path)?;
    let instance_pre = linker.instantiate_pre(&component)?;
    let pre_instantiated_component = LambdaFunctionPre::new(instance_pre)?;

    let shared_engine = &engine;
    let shared_pre_instantiated_component = &pre_instantiated_component;
    run(service_fn(move |event: LambdaEvent<Value>| async move {
        function_handler(shared_engine, shared_pre_instantiated_component, event).await
    }))
    .await
}

async fn function_handler(
    engine: &Engine,
    pre_instantiated_component: &LambdaFunctionPre<BasicState>,
    event: LambdaEvent<Value>,
) -> Result<Value, Error> {
    let ctx = WasiCtxBuilder::new()
        .inherit_stdout()
        .inherit_stderr()
        .build();

    let mut store = Store::new(
        engine,
        BasicState {
            ctx,
            table: ResourceTable::new(),
        },
    );

    let bindings = pre_instantiated_component.instantiate(&mut store)?;
    let lambda = bindings.component_function_lambda();

    let event: exports::component::function::lambda::Event = serde_json::from_value(event.payload)?;

    let response = lambda.call_handler(&mut store, event)?;

    let response_value = serde_json::to_value(response)?;
    Ok(response_value)
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
