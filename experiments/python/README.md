# Passing values between Python and WebAssembly modules

This is a hands-on exploration of invoking WebAssembly functions from Python
and transferring data back and forth using only basic WASI.

## Notes

- The stdin/stdout method described in https://petermalmgren.com/serverside-wasm-data/ 
  does not work in Python apparently https://github.com/bytecodealliance/wasmtime-py/issues/34

- How to use the linker: https://github.com/bytecodealliance/wasmtime-py/blob/main/tests/test_linker

- Understanding Wasm memory: https://radu-matei.com/blog/practical-guide-to-wasm-memory/

https://elixirforum.com/t/return-string-form-rust-in-wasm/51372/11

https://benw.is/posts/compiling-rust-to-wasi

https://bytecodealliance.github.io/wasmtime-py/

https://github.com/bytecodealliance/wasmtime/issues/9572#issuecomment-2460415021

### Multi-value returns

https://hacks.mozilla.org/2019/11/multi-value-all-the-wasm/

https://stackoverflow.com/questions/72835008/wasm-does-anyone-support-multi-value-wasm/72835009#72835009

https://github.com/rust-lang/rust/issues/73755#issuecomment-1574461575

https://github.com/ashtonmeuser/wasm-languages/blob/multi-return/Rust/src/lib.rs

https://github.com/rust-lang/rust/releases/tag/1.82.0

There is no longer any supported means to generate a module that has a function with multiple returns in WebAssembly from Rust source code. 

https://blog.rust-lang.org/2024/09/24/webassembly-targets-change-in-default-target-features.html

As a result there is no longer any possible method of writing a function in Rust that returns multiple values at the WebAssembly function type level.

https://github.com/rust-lang/rust/issues/73755#issuecomment-1577586801
