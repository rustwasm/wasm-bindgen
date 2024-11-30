use wasm_bindgen::prelude::*;

/// Manually documented function
///
/// @param {number} arg - This is my arg. It is mine.
/// @returns to whence I came
#[wasm_bindgen]
pub fn docme(arg: u32) -> u32 {
    arg + 1
}

/// Manually documented function
///
/// @param {number} arg - This is my arg. It is mine.
/// @returns to whence I came
#[wasm_bindgen(skip_jsdoc)]
pub fn docme_skip(arg: u32) -> u32 {
    arg + 1
}

/// Regular documentation.
#[wasm_bindgen]
pub fn i_has_docs(arg: u32) -> u32 {
    arg + 1
}

/// Regular documentation.
///
/// @param [b=0] Description of `arg`.
/// @param d Another description.
/// @returns
#[wasm_bindgen]
pub fn add(a: u32, b: Option<u32>, c: Option<u32>, d: Option<u32>) -> u32 {
    a + b.unwrap_or(0) + c.unwrap_or(0) + d.unwrap_or(0)
}

/// ```js
/// function foo() {
///     return 1;
/// }
/// ```
#[wasm_bindgen]
pub fn indent_test1(arg: u32) {}

/// ```js
/// function foo() {
///     return 1;
/// }
/// ```
#[wasm_bindgen(skip_jsdoc)]
pub fn indent_test2(arg: u32) {}
