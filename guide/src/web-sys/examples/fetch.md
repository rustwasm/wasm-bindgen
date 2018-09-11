# The `fetch` API

This example uses the `fetch` API to make an HTTP request to the GitHub API and
then parses the resulting JSON.

[See the full source at
`wasm-bindgen/examples/fetch`.](https://github.com/rustwasm/wasm-bindgen/tree/master/examples/fetch)

## `Cargo.toml`

The `Cargo.toml` enables a number of features related to the `fetch` API and
types used: `Headers`, `Request`, etc. It also enables `wasm-bindgen`'s `serde`
support.

```toml
{{#include ../../../../examples/fetch/Cargo.toml}}
```

## `src/lib.rs`

```rust
{{#include ../../../../examples/fetch/src/lib.rs}}
```
