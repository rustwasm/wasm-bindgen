use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn take_char_by_value(x: char) {}

#[wasm_bindgen]
pub fn return_char() -> char {
    '🚀'
}
