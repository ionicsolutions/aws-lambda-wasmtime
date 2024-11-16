.PHONY: function

function:
	cd function && cargo build --target wasm32-wasip1

release:
	cd function && cargo build --target wasm32-wasip1 --release

layer:
	rm runtime.zip || true
	zip -j runtime.zip runtime/bootstrap runtime/wasmtime

package:
	rm function.zip || true
	$(MAKE) release
	zip -j function.zip function/target/wasm32-wasip1/release/function.wasm

standalone_package:
	rm function.zip || true
	$(MAKE) release
	zip -j function.zip function/target/wasm32-wasip1/release/function.wasm runtime/bootstrap runtime/wasmtime

clean:
	cd function && cargo clean
	rm function.zip || true
	rm runtime.zip || true

get_wasmtime:
	rm runtime/wasmtime || true
	./get_wasmtime.sh
