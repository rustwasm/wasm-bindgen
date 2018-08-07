# Receiving JavaScript Closures in Exported Rust Functions

You can use the `js-sys` crate to access JavaScript's `Function` type, and
invoke that function via `Function.prototype.apply` and
`Function.prototype.call`.

For example, we can wrap a `Vec<u32>` in a new type, export it to JavaScript,
and invoke a JavaScript closure on each member of the `Vec`:

```rust
extern crate js_sys;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct VecU32 {
    xs: Vec<u32>,
}

#[wasm_bindgen]
impl VecU32 {
    pub fn each(&self, f: &js_sys::Function) {
        let this = JsValue::NULL;
        for x in &self.xs {
            let x = JsValue::from(x);
            let _ = f.call1(&this, &x);
        }
    }
}
```
