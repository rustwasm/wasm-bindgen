use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn all_optional(a: Option<u32>, b: Option<u32>, c: Option<u32>) {}

#[wasm_bindgen]
pub fn some_optional(a: Option<u32>, b: u32, c: Option<u32>) {}
