// interface-types

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn externref_in_out(a: &JsValue, b: JsValue) -> JsValue {
    b
}
