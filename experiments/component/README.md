# Understanding Wasm components

Components are pretty new (early 2024) and poorly documented.
They are not yet supported natively in Rust but require some extra tooling.

I used the introduction and reference at
[The WebAssembly Component Model](https://component-model.bytecodealliance.org/)
and a recent article by Rainer Stropek on heise.de (in German)
[WebAssembly, WASI und Rust: Dreamteam für Microservices](https://www.heise.de/hintergrund/WebAssembly-WASI-und-Rust-Dreamteam-fuer-Microservices-9978208.html).
(There is also [an English version](https://www.heise.de/en/background/WebAssembly-WASI-and-Rust-Dreamteam-for-Microservices-9978898.html).)

I learned how to add Serialize/Deserialize support to bindings for records by looking at the test suite for cargo-component:
[bytecodeallicane/cargo-component/tests/build.rs](https://github.com/bytecodealliance/cargo-component/blob/1e58afa097e153797e4195cf3f2bc749a3654f31/tests/build.rs#L753).

## Setting up

Install
[cargo-component](https://github.com/bytecodealliance/cargo-component)
and
[wac](https://github.com/bytecodealliance/wac).

## Usage

Build the function, the runtime, and assemble them into a Wasm module:

```bash
make compose
```

Run the composed module:

```bash
wasmtime composed-runtime.wasm
```

```
{"factors":[2,50227]}
```