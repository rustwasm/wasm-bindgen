use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/attributes.js")]
extern "C" {
    fn js_works();
}

#[wasm_bindgen_test]
fn works() {
    js_works();
}

#[wasm_bindgen]
#[cfg(target_arch = "wasm32")]
pub fn valid_export() -> bool {
    true
}

#[wasm_bindgen]
#[cfg(not(target_arch = "wasm32"))]
pub fn invalid_export() -> bool {
    false
}

#[wasm_bindgen]
#[cfg_attr(not(target_arch = "wasm32"), cfg(feature = "missing-feature"))]
pub fn valid_attr_export() -> bool {
    true
}

#[wasm_bindgen]
#[cfg_attr(target_arch = "wasm32", cfg(feature = "missing-feature"))]
pub fn invalid_attr_export() -> bool {
    false
}
