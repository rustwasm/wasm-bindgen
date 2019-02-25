#!/bin/sh

set -ex

# Note that typically we'd use `wasm-pack` to build the crate, but the
# `--browser` flag is very new to `wasm-bindgen` and as such doesn't have
# support in `wasm-pack` yet. Support will be added soon though!

cargo build --target wasm32-unknown-unknown --release
cargo run --manifest-path ../../crates/cli/Cargo.toml \
    --bin wasm-bindgen -- \
    ../../target/wasm32-unknown-unknown/release/without_a_bundler.wasm --out-dir pkg \
    --browser

python3 -m http.server
