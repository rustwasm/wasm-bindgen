#!/bin/sh

set -ex

# A few steps are necessary to get this build working which makes it slightly
# nonstandard compared to most other builds.
#
# * First, the Rust standard library needs to be recompiled with atomics
#   enabled. to do that we use Cargo's unstable `-Zbuild-std` feature.
#
# * Next we need to compile everything with the `atomics` and `bulk-memory`
#   features enabled, ensuring that LLVM will generate atomic instructions,
#   shared memory, passive segments, etc.
#
# * Finally, `-Zbuild-std` is still in development, and one of its downsides
#   right now is rust-lang/wg-cargo-std-aware#47 where using `rust-lld` doesn't
#   work by default, which the wasm target uses. To work around that we find it
#   and put it in PATH

RUSTFLAGS='-C target-feature=+atomics,+bulk-memory' \
  cargo build --target wasm32-unknown-unknown --release -Z build-std=std,panic_abort

# Note the usage of `--no-modules` here which is used to create an output which
# is usable from Web Workers. We notably can't use `--target bundler` since
# Webpack doesn't have support for atomics yet.
cargo run --manifest-path ../../crates/cli/Cargo.toml \
  --bin wasm-bindgen -- \
  ../../target/wasm32-unknown-unknown/release/raytrace_parallel.wasm --out-dir . \
  --no-modules

python3 -m http.server
