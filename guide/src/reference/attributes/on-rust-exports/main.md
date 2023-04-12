# `main`

When attached to the `main` function this attribute will adjust it to properly
throw errors if they can be.

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

This attribute is intended to be used on the `main` function of binaries or
examples only. Unlike `#[wasm_bindgen(start)]`, it will not cause a function to
be executed on start in a library.

Any return value that is supported by Rust is supported here, see
[`Termination`]. In order, wasm-bindgen will first detect a
`Result<(), impl Into<JsValue>>` and will throw proper `JsValue`s,
`Result<(), impl Debug>` will convert an error to a string and throw that.
Lastly anything implementing [`Termination`] will throw it's reported
[`ExitCode`](https://doc.rust-lang.org/std/process/struct.ExitCode.html) by
using it's `Debug` representation.

[`termination`]: https://doc.rust-lang.org/std/process/trait.Termination.html
