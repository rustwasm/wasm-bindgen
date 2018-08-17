cargo +nightly build --target wasm32-unknown-unknown
cargo +nightly run --manifest-path ../../crates/cli/Cargo.toml --bin wasm-bindgen -- ../../target/wasm32-unknown-unknown/debug/webaudio.wasm --out-dir .
