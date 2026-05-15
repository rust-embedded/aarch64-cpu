TARGET := aarch64-unknown-none-softfloat

default:
	cargo build --target $(TARGET)
	cargo build
	cd examples/armv8-r && cargo build

clippy:
	cargo clippy --target $(TARGET)
	cargo clippy
	cd examples/armv8-r && cargo clippy

fmt:
	cargo fmt
	cd examples/armv8-r && cargo fmt

ready: clippy fmt
	git pull
	cargo package --allow-dirty

clean:
	cargo clean
