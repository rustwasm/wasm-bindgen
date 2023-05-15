use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn take_number_slice_by_shared_ref(x: &[f64]) {}

#[wasm_bindgen]
pub fn take_number_slice_by_exclusive_ref(x: &mut [u8]) {}
