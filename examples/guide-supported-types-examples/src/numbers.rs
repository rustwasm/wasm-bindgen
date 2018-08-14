use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn take_number_by_value(x: u32) {}

#[wasm_bindgen]
pub fn return_number() -> f64 {
    42.0
}

#[wasm_bindgen]
pub fn take_option_number(x: Option<u8>) {}

#[wasm_bindgen]
pub fn return_option_number() -> Option<i16> {
    Some(-300)
}
