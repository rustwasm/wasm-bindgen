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
RUSTFLAGS='-C target-feature=+atomics,+bulk-memory' \
  xargo build --target wasm32-unknown-unknown --release

# Note the usage of `--no-modules` here which is used to create an output which
# is usable from Web Workers. We notably can't use `--target bundler` since
# Webpack doesn't have support for atomics yet.
cargo run --manifest-path ../../crates/cli/Cargo.toml \
  --bin wasm-bindgen -- \
  ../../target/wasm32-unknown-unknown/release/raytrace_parallel.wasm --out-dir . \
  --no-modules

python3 -m http.server
