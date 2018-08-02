# Testing

You can test the `web-sys` crate by running `cargo test` within the
`crates/web-sys` directory in the `wasm-bindgen` repository:

```sh
cd wasm-bindgen/crates/web-sys
cargo test --target wasm32-unknown-unknown
```

These tests all use a headless browser. See the [*Headless Browser
Tests* section for details on setup and
configuration.](../contributing.html#headless-browser-tests)

## Grouping Tests

Because headless tests can have significant setup and tear down overheads, try
and group tests together. Instead of having a different `#[test]` for every
method on some interface, have a single `#[test]` for the interface and all of
its methods. This will keep the test suite running fast, resulting in better
developer ergonomics and CI turn around times. Thanks!
