use lambda_runtime::{Error, LambdaEvent};
use serde_json::Value;
use wasi_common::pipe::{ReadPipe, WritePipe};
use wasi_common::sync::WasiCtxBuilder;
use wasi_common::WasiCtx;
use wasmtime::{Engine, Linker, Module, Store};

pub(crate) async fn function_handler(
    engine: &Engine,
    linker: &Linker<WasiCtx>,
    module: &Module,
    event: LambdaEvent<Value>,
) -> Result<Value, Error> {
    let serialized_input = event.payload.to_string();
    let stdin = ReadPipe::from(serialized_input);
    let stdout = WritePipe::new_in_memory();

    let wasi = WasiCtxBuilder::new()
        .stdin(Box::new(stdin.clone()))
        .stdout(Box::new(stdout.clone()))
        .inherit_stderr()
        .build();

    let mut store = Store::new(&engine, wasi);
    let instance = linker.instantiate(&mut store, &module)?;
    let function = instance.get_typed_func::<(), ()>(&mut store, "lambda_handler")?;

    function.call(&mut store, ())?;

    drop(store);

    let contents: Vec<u8> = stdout
        .try_into_inner()
        .map_err(|_err| anyhow::Error::msg("sole remaining reference"))?
        .into_inner();
    let response = serde_json::from_slice(&contents)?;

    Ok(response)
}
