use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn take_boxed_number_slice_by_value(x: Box<[f64]>) {}

#[wasm_bindgen]
pub fn return_boxed_number_slice() -> Box<[u32]> {
    (0..42).collect::<Vec<u32>>().into_boxed_slice()
}

#[wasm_bindgen]
pub fn take_option_boxed_number_slice(x: Option<Box<[u8]>>) {}

#[wasm_bindgen]
pub fn return_option_boxed_number_slice() -> Option<Box<[i32]>> {
    None
}
