# Converting Between JavaScript `Promise`s and Rust `Future`s

The `wasm-bindgen-futures` crate bridges the gap between JavaScript `Promise`s
and Rust `Future`s. Its `JsFuture` type provides conversion from a JavaScript
`Promise` into a Rust `Future`, and its `future_to_promise` function converts a
Rust `Future` into a JavaScript `Promise` and schedules it to be driven to
completion.

Learn more:

* [`wasm_bindgen_futures` on crates.io][crate]
* [`wasm-bindgen-futures` API documentation and example usage][docs]

[crate]: https://crates.io/crates/wasm-bindgen-futures
[docs]: https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen_futures/

## Compatibility with versions of `Future`

The current crate on crates.io, `wasm-bindgen-futures 0.4.*`, supports
`std::future::Future` and `async`/`await` in Rust. This typically requires Rust
1.39.0+ (as of this writing on 2019-09-05 it's the nightly channel of Rust).

If you're using the `Future` trait from the `futures` `0.1.*` crate then you'll
want to use the `0.3.*` track of `wasm-bindgen-futures` on crates.io.
