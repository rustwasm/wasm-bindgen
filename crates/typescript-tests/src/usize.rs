use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn usize_identity(x: usize) -> usize {
    x
}

#[wasm_bindgen]
pub fn isize_identity(x: isize) -> isize {
    x
}

#[wasm_bindgen]
pub async fn async_usize_identity(x: usize) -> usize {
    x
}

#[wasm_bindgen]
pub async fn async_isize_identity(x: isize) -> isize {
    x
}
