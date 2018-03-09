#!/bin/sh

set -ex

# This is the same as the directory above this.
cargo +nightly build --target wasm32-unknown-unknown
cargo +nightly run -p wasm-bindgen-cli --bin wasm-bindgen -- \
  ../../../target/wasm32-unknown-unknown/debug/hello_world.wasm --out-dir .

# To avoid a bug occurring when webpack, wasm, and Chrome are used together, we
# convert the .wasm module to a .js module that embeds the wasm bytecode. To
# enable this, delete hello_world_bg.wasm after building or change
# hello_world.js to import from hello_world_bg.js explicitly and uncomment
# the relevant line in index.js.
cargo +nightly run -p wasm-bindgen-cli --bin wasm2es6js -- \
  --base64 -o hello_world_bg.js hello_world_bg.wasm
# wasm2es6js --base64 -o hello_world_bg.js hello_world_bg.wasm
rm hello_world_bg.wasm

# And like the directory above this, from here it's the same.
npm install
npm run serve
