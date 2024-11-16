# Wasm runtime for AWS Lambda using `bash`, `curl`, and `wasmtime`

This is a basic AWS Lambda Wasm runtime created based on the AWS Lambda
[Building a custom runtime tutorial](https://docs.aws.amazon.com/lambda/latest/dg/runtimes-walkthrough.html).
It uses the example `bootstrap` Bash script that uses `curl` to interact with
the
[Lamda runtime API](https://docs.aws.amazon.com/lambda/latest/dg/runtimes-api.html)
and invokes the function through the `wasmtime` CLI.
The Wasm function receives the input event as a command-line argument and writes its
response to the standard output.

## Setting up

Download the `wasmtime` CLI for Linux x86_64:

```bash
make get_wasmtime
```

Verify that you can compile the example Lambda function:

```bash
make function
```

To test the function (assuming you're on Linux x86_64), run:

```bash
./runtime/bin/wasmtime function/target/wasm32-wasip1/debug/function.wasm '{"number": 123456}' 
```

This returns the prime factors of the number:
   
```
{"factors":[2,2,2,2,2,2,3,643]}
```

If you're not on Linux x86_64, you'll have to download the appropriate `wasmtime` binary
yourself. See [Installing `wasmtime`](https://docs.wasmtime.dev/cli-install.html) for details.

## Create a standalone Lambda package

Create a standalone Lambda package that includes the `wasmtime` runtime:

```bash
make standalone_package
```

This generates a `function.zip` that includes the `bootstrap` script, the
compiled function, and the `wasmtime` CLI.

When creating the Lambda function, specify the `provided.al2023` runtime
and set the handler to `function.handler` (we ignore the latter part).
Slightly increase the memory and timeout limits to 512 MB and 30 seconds
to avoid timeouts or painfully slow execution times.

```bash
aws lambda create-function \
    --function-name my-function \
    --zip-file fileb://function.zip \
    --handler function.handler \
    --runtime provided.al2023 \
    --memory-size 512 \
    --timeout 30 \
    --role arn:aws:iam::123456789012:role/lambda-role
```

## Create a Lambda layer

Create a Lambda layer that includes the `wasmtime` runtime:

```bash
make layer
```

This generates a `runtime.zip` that includes the `bootstrap` script and the
`wasmtime` CLI.

```bash
aws lambda publish-layer-version --layer-name wasmtime --zip-file fileb://runtime.zip
```

To create a function that uses the layer, run:

```bash
make package
```

This generates a `function.zip` that includes the compiled function.

When creating the Lambda function, specify the `provided.al2023` runtime
and set the handler to `function.handler` (we ignore the latter part).
Slightly increase the memory and timeout limits to 512 MB and 30 seconds
to avoid timeouts or painfully slow execution times.

```bash
aws lambda create-function \
    --function-name my-function \
    --zip-file fileb://function.zip \
    --handler function.handler \
    --runtime provided.al2023 \
    --memory-size 512 \
    --timeout 30 \
    --role arn:aws:iam::123456789012:role/lambda-role \
    --layers arn:aws:lambda:us-east-1:123456789012:layer:wasmtime:1
```

## Cleaning up

Remove build artifacts from your local directory:

```bash
make clean
```

## Related projects

* [chiefbiiko/lambda-wasmtime](https://github.com/chiefbiiko/lambda-wasmtime)
  ([blog post](https://dev.to/chiefbiiko/lambda-wasmtime-running-webassembly-on-aws-lambda-51gi))
* [revmischa/serverless-lambda-wasmtime](https://github.com/revmischa/serverless-lambda-wasmtime)
