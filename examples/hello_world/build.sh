#!/bin/sh

set -ex

# Build the `hello_world.wasm` file using Cargo/rustc
cargo +nightly build --target wasm32-unknown-unknown --release

# Run the `wasm-bindgen` CLI tool to postprocess the wasm file emitted by the
# Rust compiler to emit the JS support glue that's necessary
#
# Here we're using the version of the CLI in this repository, but for external
# usage you'd use the commented out version below
cargo +nightly run --manifest-path ../../crates/wasm-bindgen-cli/Cargo.toml \
  --bin wasm-bindgen -- \
  ../../target/wasm32-unknown-unknown/release/hello_world.wasm --out-dir .
# wasm-bindgen ../../target/wasm32-unknown-unknown/hello_world.wasm --out-dir .

# To avoid a bug occurring when webpack, wasm, and Chrome are used together, we
# convert the .wasm module to a .js module that embeds the wasm bytecode. To
# enable this, delete hello_world_wasm.wasm after building or change
# hello_world.js to import from hello_world_wasm.js explicitly and uncomment
# the relevant line in index.js.
cargo +nightly run --manifest-path ../../crates/wasm-bindgen-cli/Cargo.toml \
  --bin wasm2es6js -- \
  --base64 -o hello_world_wasm.js hello_world_wasm.wasm
# wasm2es6js --base64 -o hello_world_wasm.js hello_world_wasm.wasm

# Finally, package everything up using Webpack and start a server so we can
# browse the result
npm install
npm run serve
