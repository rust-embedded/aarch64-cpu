alias t := test

target := 'aarch64-unknown-none'
# pinned nightly version for testing
nightly := 'nightly-2026-03-09'

build:
  cargo build --target {{target}} --workspace
  cd examples/armv8-r && cargo build

test:
  rustup target add aarch64-unknown-none
  rustup target add aarch64-unknown-none-softfloat
  cd testing && cargo t --target host-tuple -- --nocapture
  rustup toolchain install {{nightly}}
  rustup component add rust-src --toolchain {{nightly}}
  cd testing && cargo +{{nightly}} t --target host-tuple -- --nocapture

clippy:
  cargo clippy --target {{target}} --lib --examples --workspace --exclude aarch64-cpu -- -D clippy::undocumented_unsafe_blocks -D missing_docs -D warnings
  cd examples/armv8-r && cargo clippy -- -D warnings

fmt:
  cargo fmt
  cd examples/armv8-r && cargo fmt

ready:
  git pull
  just clippy
  just fmt
  cargo package

clean:
  cargo clean
  cd examples/armv8-r && cargo clean
  cd testing && cargo clean
