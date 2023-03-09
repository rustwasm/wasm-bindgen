// interface-types

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn strings(a: &str) -> String {
    String::new()
}

#[wasm_bindgen]
pub fn many_strings(a: &str, b: String) {}
