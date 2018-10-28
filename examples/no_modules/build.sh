#!/bin/sh

set -ex

cargo build --target wasm32-unknown-unknown

cargo run --manifest-path ../../crates/cli/Cargo.toml \
  --bin wasm-bindgen -- \
  --no-modules \
  ../../target/wasm32-unknown-unknown/debug/no_modules.wasm --out-dir .

python -m SimpleHTTPServer
