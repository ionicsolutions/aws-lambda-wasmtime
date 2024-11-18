mod bindings;
use bindings::component::function::lambda::{handler, Event};

fn main() {
    let event = Event { number: 100454 };
    let response = handler(event);
    println!("{}", serde_json::to_string(&response).unwrap());
}
