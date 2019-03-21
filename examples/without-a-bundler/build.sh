#!/bin/sh

set -ex

wasm-pack build --target web
python3 -m http.server
