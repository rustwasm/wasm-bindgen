#!/bin/sh

set -ex

# Two critical steps are required here to get this working:
#
# * First, the Rust standard library needs to be compiled. The default version
#   is not compatible with atomics so we need to compile a version, with xargo,
#   that is compatible.
#
# * Next we need to compile everything with the `atomics` feature enabled,
#   ensuring that LLVM will generate atomic instructions and such.
RUSTFLAGS='-C target-feature=+atomics' \
  rustup run nightly xargo build --target wasm32-unknown-unknown --release

# Threading support is disabled by default in wasm-bindgen, so use an env var
# here to turn it on for our bindings generation. Also note that webpack isn't
# currently compatible with atomics, so we go with the --no-modules output.
WASM_BINDGEN_THREADS=1 \
  cargo +nightly run --manifest-path ../../crates/cli/Cargo.toml \
    --bin wasm-bindgen -- \
    ../../target/wasm32-unknown-unknown/release/raytrace_parallel.wasm --out-dir . \
    --no-modules

python3 -m http.server
