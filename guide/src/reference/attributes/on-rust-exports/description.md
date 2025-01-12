# `return_description` and `param_description`

Descriptions to return and parameter documentation can be added with `#[wasm_bindgen(return_description)]` and `#[wasm_bindgen(param_description)]`.

```rust
/// Adds `arg1` and `arg2`.
#[wasm_bindgen(return_description = "the result of the addition of `arg1` and `arg2`")]
pub fn add(
    #[wasm_bindgen(param_description = "the first number")]
    arg1: u32,
    #[wasm_bindgen(param_description = "the second number")]
    arg2: u32,
) -> u32 {
    arg1 + arg2
}

#[wasm_bindgen]
pub struct FooList {
    // properties
}

#[wasm_bindgen]
impl FooList {
    /// Returns the number at the given index.
    #[wasm_bindgen(return_description = "the number at the given index")]
    pub fn number(
        &self,
        #[wasm_bindgen(param_description = "the index of the number to be returned")]
        index: u32,
    ) -> u32 {
        // function body
    }
}
```

Which will generate the following JS bindings:
```js
/**
 * Adds `arg1` and `arg2`.
 *
 * @param {number} arg1 - the first number
 * @param {number} arg2 - the second number
 * @returns {number} the result of the addition of `arg1` and `arg2`
 */
export function add(arg1, arg2) {
    // ...
}

export class FooList {
    /**
     * Returns the number at the given index.
     *
     * @param {number} index - the index of the number to be returned
     * @returns {number} the number at the given index
     */
    number(index) {
        // ...
    }
}
```
