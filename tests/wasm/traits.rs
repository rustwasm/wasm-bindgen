use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/traits.js")]
extern "C" {
    fn js_works();
}

#[wasm_bindgen]
pub trait Testable {
    /// test
    fn method(n: i32) -> u32;
    fn second(&self, n: i32) -> u32;
}

#[wasm_bindgen_test]
fn works() {
    js_works();
}
