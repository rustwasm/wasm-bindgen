use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add_u32(a: u32, b: u32) -> u32 {
    a + b
}

#[wasm_bindgen]
pub fn add_i32(a: i32, b: i32) -> i32 {
    a + b
}
