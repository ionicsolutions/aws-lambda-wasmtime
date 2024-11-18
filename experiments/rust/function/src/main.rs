use lambda_runtime::{run, service_fn, tracing, Error};
mod wasm_handler;
use wasm_handler::function_handler;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
