use std::ptr;
use std::ptr::NonNull;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub unsafe fn take_pointer_by_value(x: Option<NonNull<u8>>) {
    Box::from_raw(x.unwrap().as_ptr());
}

#[wasm_bindgen]
pub fn return_pointer() -> Option<NonNull<u8>> {
    Some(NonNull::from(Box::leak(Box::new(42))))
}
