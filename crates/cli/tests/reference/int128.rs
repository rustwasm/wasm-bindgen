use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn echo_i128(a: i128) -> i128 {
    a
}
#[wasm_bindgen]
pub fn echo_u128(a: u128) -> u128 {
    a
}

#[wasm_bindgen]
pub fn echo_option_i128(a: Option<i128>) -> Option<i128> {
    a
}
#[wasm_bindgen]
pub fn echo_option_u128(a: Option<u128>) -> Option<u128> {
    a
}
