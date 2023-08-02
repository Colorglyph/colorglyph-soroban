default: build

test: build
	cargo test

build:
	soroban contract build

build-opt: build
	soroban contract optimize --wasm target/wasm32-unknown-unknown/release/colorglyph.wasm

fmt:
	cargo fmt --all

clean:
	rm -rf colorglyph-sdk
	cargo clean
