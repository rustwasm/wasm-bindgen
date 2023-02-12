# `main`

When attached to the `fn main()` function this attribute will adjust it to
properly throw errors if they can be. This is only intended to be used for
binaries.

```rust
#[wasm_bindgen(main)]
fn main() -> Result<(), JsValue> {
    Err(JsValue::from("this error message will be thrown"))
}
```

The attribute also allows using `async fn main()` in Cargo binaries.

```rust
#[wasm_bindgen(main)]
async fn main() {
    // ...
    future.await;
}
```

Unlike `#[wasm_bindgen(start)]` this will not export a function to be executed
on startup, it should only be used in Cargo binaries or examples for the `main`
function. `#[wasm_bindgen(start)]` will prevent the `main` function to start and
should not be used in conjunction.

Any return value that is supported by Rust is supported here, see
[`Termination`]. In order, wasm-bindgen will first detect a
`Result<(), impl Into<JsValue>>` and will throw proper `JsValue`s,
`Result<(), impl Debug>` will convert an error to a string and throw that.
Lastly anything implementing [`Termination`] will throw it's reported
[`ExitCode`](https://doc.rust-lang.org/std/process/struct.ExitCode.html) by
using it's `Debug` representation.

[`termination`]: https://doc.rust-lang.org/std/process/trait.Termination.html
