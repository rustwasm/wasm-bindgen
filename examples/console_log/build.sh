#!/bin/sh

# For more comments about what's going on here, see the `hello_world` example

set -ex

cargo +nightly build --target wasm32-unknown-unknown
cargo run --manifest-path ../../crates/cli/Cargo.toml \
  --bin wasm-bindgen -- \
  ../../target/wasm32-unknown-unknown/debug/console_log.wasm --out-dir .
npm install
npm run serve
