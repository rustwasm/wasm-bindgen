#!/bin/sh

set -ex

cargo build --target wasm32-unknown-unknown

rm -rf pkg
mkdir pkg
cargo run -p wasm-bindgen-cli --bin wasm-bindgen -- \
  ../../target/wasm32-unknown-unknown/debug/typescript_tests.wasm \
  --out-dir pkg \
  --typescript

if [ ! -d node_modules ]; then
  npm install
fi

npm run tsc
