# Use of `static` to Access JS Objects

JavaScript modules will often export arbitrary static objects for use with
their provided interfaces. These objects can be accessed from Rust by declaring
a named `static` in the `extern` block with an
`#[wasm_bindgen(thread_local_v2)]` attribute. `wasm-bindgen` will bind a
`JsThreadLocal` for these objects, which can be cloned into a `JsValue`. For
example, given the following JavaScript:

```js
let COLORS = {
    red: 'rgb(255, 0, 0)',
    green: 'rgb(0, 255, 0)',
    blue: 'rgb(0, 0, 255)',
};
```

`static` can aid in the access of this object from Rust:

```rust
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(thread_local_v2)]
    static COLORS;
}

fn get_colors() -> JsValue {
    COLORS.with(JsValue::clone)
}
```

Since `COLORS` is effectively a JavaScript namespace, we can use the same
mechanism to refer directly to namespaces exported from JavaScript modules, and
even to exported classes:

```js
let namespace = {
    // Members of namespace...
};

class SomeType {
    // Definition of SomeType...
};

export { SomeType, namespace };
```

The binding for this module:

```rust
#[wasm_bindgen(module = "/js/some-rollup.js")]
extern "C" {
    // Likewise with the namespace--this refers to the object directly.
    #[wasm_bindgen(thread_local_v2, js_name = namespace)]
    static NAMESPACE: JsValue;

    // Refer to SomeType's class
    #[wasm_bindgen(thread_local_v2, js_name = SomeType)]
    static SOME_TYPE: JsValue;

    // Other bindings for SomeType
    type SomeType;
    #[wasm_bindgen(constructor)]
    fn new() -> SomeType;
}
```

## Static strings

Strings can be imported to avoid going through `TextDecoder/Encoder` when requiring just a `JsString`. This can be useful when dealing with environments where `TextDecoder/Encoder` is not available, like in audio worklets.

```rust
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(thread_local_v2, static_string)]
    static STRING: JsString = "a string literal";
}
```
