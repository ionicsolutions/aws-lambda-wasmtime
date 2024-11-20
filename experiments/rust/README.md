# Wasm runtime for AWS Lambda using Rust and `wasmtime`

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

We create a new store and instance for each invocation,
but share the engine, the linker, and the module.
Per
[this comment by Nick Fitzgerald](https://github.com/bytecodealliance/wasmtime/issues/9572#issuecomment-2460415021)
this seems to be the best practice.
State sharing is implemented following the pattern shown in the
[Using shared state](https://docs.aws.amazon.com/lambda/latest/dg/rust-handler.html#rust-shared-state)
section in the AWS Lambda documentation for the Rust runtime.

## Setting up

Install [Cargo Lambda](https://www.cargo-lambda.info/guide/installation.html)
and verify that you can compile the example Wasm module:

```bash
make function
```

## Test locally

Use the `watch` command to launch a local instance of the Lambda function:

```bash
make watch
```

Then, in a second terminal window, call the Lambda function:

```bash
cargo lambda invoke runtime --data-ascii '{"number": 1237789}'
```

This returns the prime factors of the number:
   
```
{"factors":[7,7,25261]}
```

## Cleaning up

Remove build artifacts from your local directory:

```bash
make clean
```
