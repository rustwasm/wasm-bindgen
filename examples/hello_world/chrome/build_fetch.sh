#!/bin/sh

set -ex

# This is the same build.sh, later we are going to use the
# fetch flag to avoid including the wasm module as a string
# of base64, instead using the js `fetch` function 
#to request the module from the server.
cargo +nightly build --target wasm32-unknown-unknown
cargo +nightly run -p wasm-bindgen-cli --bin wasm-bindgen -- \
  ../../../target/wasm32-unknown-unknown/debug/hello_world.wasm --out-dir .

# To avoid a bug occurring when webpack, wasm, and Chrome are used together, we
# create a .js module that will download the .wasm module via fetch. 
cargo +nightly run -p wasm-bindgen-cli --bin wasm2es6js -- \
  --fetch ./hello_world_bg.wasm -o hello_world_bg.js hello_world_bg.wasm

# Install the npm items as usual.
npm install

# since we kept the same name for the .js module, we need
# to force webpack to ignore any other file extensions
npm run serve -- --resolve-extensions .js
