.PHONY: function

function:
	cd function && cargo build --target wasm32-wasip1

release:
	cd function && cargo build --target wasm32-wasip1 --release

watch:
	$(MAKE) release
	cd runtime && WASM_MODULE_PATH="../function/target/wasm32-wasip1/release/function.wasm" cargo lambda watch

clean:
	cd function && cargo clean

fmt:
	cd function && cargo fmt
	cd runtime && cargo fmt