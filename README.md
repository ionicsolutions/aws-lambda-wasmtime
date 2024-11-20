# Wasm runtime for AWS Lambda

Running
[WebAssembly components](https://component-model.bytecodealliance.org/)
on
[AWS Lambda](https://docs.aws.amazon.com/lambda/)
with the
[`wasmtime`](https://wasmtime.dev/) runtime.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install)
- [cargo lambda](https://www.cargo-lambda.info/guide/installation.html)
- [cargo component](https://github.com/bytecodealliance/cargo-component)

## Quickstart

Compile and launch a local instance of a Lambda function
running the [_demo component_](demo-component) that calculates
the prime factors of a number:

```bash
make demo
```

In a second terminal window, call the Lambda function:

```bash
cargo lambda invoke runtime --data-ascii '{"number": 1237789}'
```

This returns the prime factors of the number:

```
{"factors":[7,7,25261]}
```

To clean up, remove all generated build artifacts with:

```bash
make clean
```

## Usage

The AWS Lambda Wasm runtime expects a component with the following interface:

```wit
interface lambda {

    record event {
        // ...
    }

    record response {
        // ...
    }

    handler: func(event: event) -> response;
}
```

See the section
[Getting Started](https://github.com/bytecodealliance/cargo-component?tab=readme-ov-file#getting-started)
in the `cargo component` README for instructions on how to create a component from scratch,
or refer to the example in the [_demo-component_](demo-component) directory in this repository.

Build the component with

```bash
cargo component build --release
```

to create a Wasm component under _target/wasm32-wasip1/release_.

In `runtime/Cargo.toml`, set the `component_world` key in `[package.metadata]`
to the path to the Wasm component's _*.wit_ file.

Then, compile the runtime with

```bash
cargo lambda build --release --compiler cargo  # Linux x86_64
cargo lambda build --release # all other platforms (cross-compiles with Zig)
```

This generates the _target/lambda/release/bootstrap_ binary required by AWS Lambda.

For testing, set the `_HANDLER` environment variable
to the relative path to the compiled Wasm component
and launch a local Lambda function instance use `cargo lambda watch`:

```bash
export _HANDLER="../demo-component/target/wasm32-wasip1/release/function.wasm"
cargo lambda watch
```

To call the Lambda function, use `cargo lambda invoke`,
replacing `--data-ascii` with a valid input payload:

```bash
cargo lambda invoke runtime --data-ascii '{"number": 123}'
```

To create a ZIP package for deployment to AWS Lambda,
combine the `bootstrap` binary with the Wasm component:

```bash
zip -j function.zip target/lambda/release/bootstrap demo-component/target/wasm32-wasip1/release/function.wasm
```

Upload the ZIP package to AWS Lambda via the AWS Management Console or the AWS CLI:

```bash
aws lambda create-function \
    --function-name my-function \
    --zip-file fileb://function.zip \
    --handler function.wasm \  # full file name of the Wasm component
    --runtime provided.al2023 \
    --role arn:aws:iam::123456789012:role/lambda-role  # replace with valid execution role
```

You can also package the runtime as a Lambda layer
and only include the component in the ZIP archive.
Note that a runtime is specific to a particular component interface.
