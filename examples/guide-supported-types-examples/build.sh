#!/bin/sh

# For more comments about what's going on here, see the `hello_world` example

set -ex

cargo +nightly build --target wasm32-unknown-unknown --release
cargo +nightly run --manifest-path ../../crates/cli/Cargo.toml \
  --bin wasm-bindgen -- \
  ../../target/wasm32-unknown-unknown/release/guide_supported_types_examples.wasm --out-dir .
npm install
npm run serve
