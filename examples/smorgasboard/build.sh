#!/bin/sh

set -ex

cargo +nightly build --target wasm32-unknown-unknown --release

# Here we're using the version of the CLI in this repository, but for external
# usage you'd use the commented out version below
cargo +nightly run --manifest-path ../../crates/wasm-bindgen-cli/Cargo.toml \
  --bin wasm-bindgen -- \
  ../../target/wasm32-unknown-unknown/release/smorgasboard.wasm --out-dir .
# wasm-bindgen ../../target/wasm32-unknown-unknown/hello_world.wasm --out-dir .

npm install
npm run serve
