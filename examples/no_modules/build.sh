#!/bin/sh

set -ex

cargo +nightly build --target wasm32-unknown-unknown

cargo +nightly run --manifest-path ../../crates/cli/Cargo.toml \
  --bin wasm-bindgen -- \
  --no-modules \
  ../../target/wasm32-unknown-unknown/debug/no_modules.wasm --out-dir .

python -m SimpleHTTPServer
