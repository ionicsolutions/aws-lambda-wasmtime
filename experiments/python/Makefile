.PHONY: function

function:
	cd function && cargo build --target wasm32-wasip1

release:
	cd function && cargo build --target wasm32-wasip1 --release

clean:
	cd function && cargo clean
	rm output.txt || true
