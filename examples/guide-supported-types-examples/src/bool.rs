use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn take_bool_by_value(x: bool) {}

#[wasm_bindgen]
pub fn return_bool() -> bool {
    true
}

#[wasm_bindgen]
pub fn take_option_bool(x: Option<bool>) {}

#[wasm_bindgen]
pub fn return_option_bool() -> Option<bool> {
    Some(false)
}
