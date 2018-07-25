# wasm-bindgen-futures

[Documention][documentation]

This is an experimental crate (aka just written) which is targeted at bridging
a Rust `Future` and a JS `Promise`. Internally it contains two conversions, one
from a JS `Promise` to a Rust `Future`, and another from a Rust `Future` to a
JS `Promise`.

See the [documentation] for more info.

[documentation]: https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen_futures/
