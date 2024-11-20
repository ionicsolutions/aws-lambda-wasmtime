.PHONY: runtime

component:
	cd demo-component && cargo component build --release

runtime:
	$(MAKE) function
	cd runtime && cargo component build

demo:
	$(MAKE) component
	cd runtime && _HANDLER="../demo-component/target/wasm32-wasip1/release/function.wasm" cargo lambda watch

clean:
	cd runtime && cargo clean
	cd demo-component && cargo clean

fmt:
	cd runtime && cargo fmt
	cd demo-component && cargo fmt