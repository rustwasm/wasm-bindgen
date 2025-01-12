# `unchecked_return_type` and `unchecked_param_type`

Return and parameter types of exported functions and methods can be overwritten with `#[wasm_bindgen(unchecked_return_type)]` and `#[wasm_bindgen(unchecked_param_type)]`.

> **Note**: Types that are provided using `#[wasm_bindgen(unchecked_return_type)]` and `#[wasm_bindgen(unchecked_param_type)]` aren't checked for their contents. They will end up in a function signature and JSDoc exactly as they have been specified. E.g. `#[wasm_bindgen(unchecked_return_type = "number")]` on a function returning `String` will return a `string`, not a `number`, even if the TS signature and JSDoc will say otherwise.

```rust
#[wasm_bindgen(unchecked_return_type = "Foo")]
pub fn foo(
    #[wasm_bindgen(unchecked_param_type = "Bar")]
    arg1: JsValue,
) -> JsValue {
    // function body
}

#[wasm_bindgen]
pub struct Foo {
    // properties
}

#[wasm_bindgen]
impl Foo {
    #[wasm_bindgen(unchecked_return_type = "Baz")]
    pub fn foo(
        &self,
        #[wasm_bindgen(unchecked_param_type = "Bar")]
        arg1: JsValue,
    ) -> JsValue {
        // function body
    }
}
```

Which will generate the following JS bindings:
```js
/**
 * @param {Bar} arg1
 * @returns {Foo}
 */
export function foo(arg1) {
    // ...
}

export class Foo {
    /**
     * @param {Bar} arg1
     * @returns {Baz}
     */
    foo(arg1) {
        // ...
    }
}
```
