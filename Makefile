.PHONY: runtime

component:
	cd demo-component && cargo component build --release

runtime:
	$(MAKE) component
	cd runtime && cargo lambda build --release --compiler cargo

demo:
	$(MAKE) component
	cd runtime && _HANDLER="../demo-component/target/wasm32-wasip1/release/function.wasm" cargo lambda watch

clean:
	cd runtime && cargo clean
	cd demo-component && cargo clean

fmt:
	cd runtime && cargo fmt
	cd demo-component && cargo +nightly fmt