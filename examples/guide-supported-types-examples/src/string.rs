use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn take_string_by_value(x: String) {}

#[wasm_bindgen]
pub fn return_string() -> String {
    "hello".into()
}

#[wasm_bindgen]
pub fn take_option_string(x: Option<String>) {}

#[wasm_bindgen]
pub fn return_option_string() -> Option<String> {
    None
}
