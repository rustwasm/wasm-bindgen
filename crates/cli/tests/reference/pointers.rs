use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn const_pointer(input: *const u8) -> *const u8 {
    u32::MAX as *const _
}

#[wasm_bindgen]
pub fn mut_pointer(input: *mut u8) -> *mut u8 {
    u32::MAX as *mut _
}
