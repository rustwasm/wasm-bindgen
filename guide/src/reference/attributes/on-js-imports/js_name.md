# `js_name = blah`

The `js_name` attribute can be used to bind to a different function in
JavaScript than the identifier that's defined in Rust.

Most often, this is used to convert a camel-cased JavaScript identifier into a
snake-cased Rust identifier:

```rust
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = jsOftenUsesCamelCase)]
    fn js_often_uses_camel_case() -> u32;
}
```

Sometimes, it is used to bind to JavaScript identifiers that are not valid Rust
identifiers, in which case `js_name = "some string"` is used instead of `js_name
= ident`:

```rust
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "$$$")]
    fn cash_money() -> u32;
}
```
However, you can also use `js_name` to define multiple signatures for
polymorphic JavaScript functions:

```rust
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn console_log_str(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn console_log_u32(n: u32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn console_log_many(a: u32, b: &JsValue);
}
```

All of these functions will call `console.log` in JavaScript, but each
identifier will have only one signature in Rust.

Note that if you use `js_name` when importing a type you'll also need to use the
[`js_class` attribute][jsclass] when defining methods on the type:

```rust
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = String)]
    type JsString;
    #[wasm_bindgen(method, getter, js_class = "String")]
    pub fn length(this: &JsString) -> u32;
}
```

[jsclass]: js_class.html
