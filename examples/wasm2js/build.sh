#!/bin/sh

set -ex

# Compile our wasm module and run `wasm-bindgen`
cargo build --target wasm32-unknown-unknown --release
cargo run --manifest-path ../../crates/cli/Cargo.toml \
  --bin wasm-bindgen -- \
  ../../target/wasm32-unknown-unknown/release/wasm2js.wasm --out-dir .

# Run the `wasm2js` tool from `binaryen`
wasm2js wasm2js_bg.wasm -o wasm2js_bg.js

# Move our original wasm out of the way to avoid cofusing Webpack.
mv wasm2js_bg.wasm wasm2js_bg.bak.wasm

npm install
npm run serve
