use wasm_bindgen::prelude::*;

/// Manually documented function
///
/// @param {number} arg - This is my arg. It is mine.
/// @returns to whence I came
#[wasm_bindgen(skip_jsdoc)]
pub fn docme(arg: u32) -> u32 {
    arg + 1
}
