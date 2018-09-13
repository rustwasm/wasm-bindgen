#!/bin/sh

# For more coments about what's going on here, see the `hello_world` example

set -ex
cd "$(dirname $0)"

cargo +nightly build --target wasm32-unknown-unknown

cargo +nightly run --manifest-path ../../crates/cli/Cargo.toml \
  --bin wasm-bindgen -- \
  ../../target/wasm32-unknown-unknown/debug/wasm_bindgen_paint.wasm --out-dir .

npm install
npm run serve
