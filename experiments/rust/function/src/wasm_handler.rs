use lambda_runtime::{Error, LambdaEvent};
use serde_json::{Value, json};

pub(crate) async fn function_handler(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let command = event.payload.to_string();

    let resp = json!({
        "req_id": event.context.request_id,
        "msg": format!("Command {}.", command),
    });

    Ok(resp)
}
