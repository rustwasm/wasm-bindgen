#!/bin/sh

set -ex

# Compile our wasm module and run `wasm-bindgen`
wasm-pack build --target web

# Run the `wasm2js` tool from `binaryen`
wasm2js pkg/wasm2js_bg.wasm -o pkg/wasm2js_bg.js

# Update our JS shim to require the JS file instead
sed -i 's/wasm2js_bg.wasm/wasm2js_bg.js/' pkg/wasm2js.js

http
