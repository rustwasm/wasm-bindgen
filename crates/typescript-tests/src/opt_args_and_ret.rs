use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn opt_fn(_a: Option<i32>) -> Option<i32> {
    None
}
