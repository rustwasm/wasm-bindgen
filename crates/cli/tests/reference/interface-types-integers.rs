// interface-types

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn integers(_a1: u8, _a2: i8, _a3: u16, _a4: i16, _a5: u32, _a6: i32, _a7: f32, _a8: f64) {}

#[wasm_bindgen]
pub fn ret_i8() -> i8 {
    0
}

#[wasm_bindgen]
pub fn ret_u8() -> u8 {
    1
}

#[wasm_bindgen]
pub fn ret_i16() -> i16 {
    2
}

#[wasm_bindgen]
pub fn ret_u16() -> u16 {
    3
}

#[wasm_bindgen]
pub fn ret_i32() -> i32 {
    4
}

#[wasm_bindgen]
pub fn ret_u32() -> u32 {
    5
}

#[wasm_bindgen]
pub fn ret_f32() -> f32 {
    6.0
}

#[wasm_bindgen]
pub fn ret_f64() -> f64 {
    7.0
}
