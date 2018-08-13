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
