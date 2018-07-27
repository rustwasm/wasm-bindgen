# Testing

You can test the `js-sys` crate by running `cargo test --target
wasm32-unknown-unknown` within the `crates/web-sys` directory in the
`wasm-bindgen` repository:

```sh
cd wasm-bindgen/crates/web-sys
cargo test --target wasm32-unknown-unknown
```

These tests are largely executed in Node.js right now via the
`wasm-bindgen-test` framework. More documentation on the framework coming soon!
