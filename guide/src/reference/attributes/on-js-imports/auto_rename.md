# `auto_rename`

> **Note**: This attribute is only available `wasm-bindgen` in versions 0.2.96 and later.

> **Note**: See the page about [automatic renaming](../../automatic-renaming.md.md) for more general information about this attribute.

`wasm-bindgen` generally encourages users to follow Rust naming conventions and use `js_name` to specify the JavaScript name of functions, types, etc. However, this can be very tedious since JavaScript typically uses camelCase for function names, requiring the user to specify the `js_name` attribute for almost every function.

The `auto_rename` attribute makes this easier by automatically setting the `js_name` of functions to the camelCase version of the Rust function name. This is useful when the Rust function name is in snake_case and the JavaScript function name is in camelCase.

Example:

```rust
#[wasm_bindgen(auto_rename, js_namespace = document)]
extern "C" {
    // the JS name will be inferred as `createElement`
    fn create_element(tag_name: String) -> JsValue;

    // the JS name will be inferred as `createElementWithOptions`, so it has
    // to be set explicitly using the `js_name` attribute
    #[wasm_bindgen(js_name = createElement)]
    fn create_element_with_options(tag_name: String, options: JsValue) -> JsValue;

    // since the function name is not in snake_case, the `auto_rename` attribute
    // has no effect, and the JS name will be inferred as `documentElement`
    //
    // Note: This is *NOT* recommended, as it is not idiomatic Rust. Always use
    // snake_case for function names in Rust.
    #[wasm_bindgen(getter)]
    fn documentElement() -> JsValue;
}
```

> **Note**: The `auto_rename` attribute is valid on `extern` blocks, but not on individual functions inside `extern` blocks.
