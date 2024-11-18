# Wasm runtime for AWS Lambda using `wasmtime`

This PoC uses the
[Rust runtime for AWS Lambda](https://github.com/awslabs/aws-lambda-rust-runtime)
to run WebAssembly modules.

Communication between the Wasm module and Lambda happens via stdin/stdout,
as described in
[Getting data in and out of WASI modules](https://petermalmgren.com/serverside-wasm-data/)
by Peter Malmgren and similar to
[the WasmEdge AWS Lambda example](https://wasmedge.org/docs/start/usage/serverless/aws)
(see also
[second-state/aws-lambda-wasm-runtime](https://github.com/second-state/aws-lambda-wasm-runtime)).

The downside is that we deserialize and serialize the data twice,
because the AWS Lambda Rust runtime deserializes the input payload
prior to invoking the handler function.
