#!/bin/sh

set -ex

# Compile our wasm module and run `wasm-bindgen`
wasm-pack build

# Run the `wasm2js` tool from `binaryen`
wasm2js pkg/wasm2js_bg.wasm -o pkg/wasm2js_bg.js

# Move our original wasm out of the way to avoid cofusing Webpack.
mv pkg/wasm2js_bg.wasm pkg/wasm2js_bg.bak.wasm

npm install
npm run serve
