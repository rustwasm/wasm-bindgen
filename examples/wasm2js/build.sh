#!/bin/sh

set -ex

# Compile our wasm module and run `wasm-bindgen`
wasm-pack build --target web

# Run the `wasm2js` tool from `binaryen`
wasm2js pkg/wasm2js_bg.wasm -o pkg/wasm2js_bg.js

# Update our JS shim to import the JS file instead
sed -i "s,^let wasm;\$,import * as wasm from './wasm2js_bg.js';," pkg/wasm2js.js
# Update the wasm2js js to import from the shim
sed -i "s,from 'wbg';\$,from \'./wasm2js.js\';," pkg/wasm2js_bg.js

http
