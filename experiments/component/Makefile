.PHONY: function runtime generator

function:
	cd function && cargo component build

runtime:
	$(MAKE) function
	cd runtime && cargo component build

compose:
	$(MAKE) runtime
	wac plug runtime/target/wasm32-wasip1/debug/runtime.wasm --plug function/target/wasm32-wasip1/debug/function.wasm -o ./composed-runtime.wasm

generator:
	$(MAKE) function
	cd generator && cargo build

clean:
	cd function && cargo clean
	cd runtime && cargo clean
	cd generator && cargo clean

fmt:
	cd function && cargo fmt
	cd runtime && cargo fmt
	cd generator && cargo fmt