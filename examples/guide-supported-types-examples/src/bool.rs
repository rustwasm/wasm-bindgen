use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn take_bool_by_value(x: bool) {}

#[wasm_bindgen]
pub fn return_bool() -> bool {
    true
}
