use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn foo(a: &str) {
    drop(a);
}

#[wasm_bindgen]
pub fn bar(a: &str, b: String, ptr: u32, len: u32) -> String {
    b
}
