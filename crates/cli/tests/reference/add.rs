use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add_u32(a: u32, b: u32) -> u32 {
    a + b
}

#[wasm_bindgen]
pub fn add_i32(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn add_u64(a: u64, b: u64) -> u64 {
    a + b
}

#[wasm_bindgen]
pub fn add_i64(a: i64, b: i64) -> i64 {
    a + b
}

#[wasm_bindgen]
pub fn add_option_u64(a: Option<u64>, b: u64) -> Option<u64> {
    a.map(|a| a + b)
}

#[wasm_bindgen]
pub fn add_option_i64(a: Option<i64>, b: i64) -> Option<i64> {
    a.map(|a| a + b)
}
