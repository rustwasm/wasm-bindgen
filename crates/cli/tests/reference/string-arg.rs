use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn foo(a: &str) {
    drop(a);
}

#[wasm_bindgen]
pub fn two_strings(a: &str, b: String) -> String {
    b
}

#[wasm_bindgen]
pub fn two_strings_and_some_more(a: &str, b: String, ptr: u32, len: u32) -> String {
    b
}
