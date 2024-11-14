# `auto_rename`

> **Note**: This attribute is only available `wasm-bindgen` in versions 0.2.96 and later.

> **Note**: See the page about [automatic renaming](../../automatic-renaming.md.md) for more general information about this attribute.

`wasm-bindgen` generally encourages users to follow Rust naming conventions and use `js_name` to specify the JavaScript name of functions, types, etc. However, this can be very tedious since JavaScript typically uses camelCase for function names, requiring the user to specify the `js_name` attribute for almost every function.

The `auto_rename` attribute makes this easier by automatically setting the `js_name` of functions/methods/field to the camelCase version of the Rust function name. This is useful when the Rust function name is in snake_case and the JavaScript function name is in camelCase.

Example:

```rust
#[wasm_bindgen(auto_rename)]
pub struct MetalHead {
    // the JS name will be inferred as `songsWritten`
    pub songs_written: u32,
    // the JS name will be inferred as `headbangsPerMinute`
    pub headbangs_per_minute: f64,
}

#[wasm_bindgen(auto_rename)]
impl MetalHead {
    // the JS name will be inferred as `powerLevel`
    #[wasm_bindgen(getter)]
    pub fn power_level(&self) -> f64 {
        self.songs_written as f64 * self.headbangs_per_minute
    }

    // the JS name will be inferred as `enterMoshpit`
    pub fn enter_moshpit(&self) {
        // ...
    }

    // since the function name is not in snake_case, the `auto_rename` attribute
    // has no effect, and the JS name will be inferred as `SCREAM`
    //
    // Note: This is *NOT* recommended, as it is not idiomatic Rust. Always use
    // snake_case for function names in Rust.
    pub fn SCREAM(&self) {
        // ...
    }
}
```

> **Note**: The `auto_rename` attribute is valid on `struct`s and `impl` blocks, but not on individual functions or fields.
