use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/string_vecs.js")]
extern "C" {
    fn pass_string_vec();
    fn pass_invalid_string_vec();
}

#[wasm_bindgen]
pub fn consume_string_vec(_: Vec<String>) {}

#[wasm_bindgen_test]
fn test_valid() {
    pass_string_vec();
}

#[wasm_bindgen_test]
fn test_invalid() {
    pass_invalid_string_vec();
}
