default: build

test: build
	cargo test

build:
	cargo build --target wasm32-unknown-unknown --release

fmt:
	cargo fmt --all

clean:
	cargo clean
