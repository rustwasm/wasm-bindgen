#!/bin/sh

set -ex

# Compile our wasm module
cargo +nightly build --target wasm32-unknown-unknown --release

# Run wasm-bindgen, and note that the `--no-demangle` argument is here is
# important for compatibility with wasm2js!
cargo +nightly run --manifest-path ../../crates/cli/Cargo.toml \
  --bin wasm-bindgen -- \
  --no-demangle \
  ../../target/wasm32-unknown-unknown/release/asmjs.wasm --out-dir .

# Run the `wasm2js` tool from `binaryen`
wasm2js asmjs_bg.wasm -o asmjs_bg.js

# Move our original wasm out of the way to avoid cofusing Webpack.
mv asmjs_bg.wasm asmjs_bg.bak.wasm

npm install
npm run serve
