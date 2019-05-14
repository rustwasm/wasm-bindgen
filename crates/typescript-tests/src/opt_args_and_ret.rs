use wasm_bindgen::prelude::*;

#[wasm_bindgen]
/// Optional parameters followed by non-optional parameters.
/// Only the parameter _c may be marked as omittable.
pub fn opt_fn_mixed(_a: Option<i32>, _b: i32, _c: Option<i32>) -> Option<i32> {
    None
}

#[wasm_bindgen]
/// Only optional parameters. All of them may be marked as omittable.
pub fn opt_fn_only(_a: Option<i32>, _b: Option<i32>, _c: Option<i32>) -> Option<i32> {
    None
}
