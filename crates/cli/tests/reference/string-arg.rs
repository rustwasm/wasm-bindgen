use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn foo(a: &str) {
    drop(a);
}
