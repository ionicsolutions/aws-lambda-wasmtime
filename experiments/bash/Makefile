.PHONY: function

function:
	cd function && cargo build --target wasm32-wasip1

release:
	cd function && cargo build --target wasm32-wasip1 --release

layer:
	rm runtime.zip || true
	cd runtime && zip -r ../runtime.zip .

package:
	rm function.zip || true
	$(MAKE) release
	zip -j function.zip function/target/wasm32-wasip1/release/function.wasm

standalone_package:
	rm function.zip || true
	$(MAKE) release
	zip -j function.zip function/target/wasm32-wasip1/release/function.wasm runtime/bootstrap runtime/bin/wasmtime

clean:
	cd function && cargo clean
	rm function.zip || true
	rm runtime.zip || true

get_wasmtime:
	rm runtime/bin/wasmtime || true
	./get_wasmtime.sh
