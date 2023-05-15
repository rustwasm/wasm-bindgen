use std::ptr;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn take_pointer_by_value(x: *mut u8) {}

#[wasm_bindgen]
pub fn return_pointer() -> *mut u8 {
    ptr::null_mut()
}
