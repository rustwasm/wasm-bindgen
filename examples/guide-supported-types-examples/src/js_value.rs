use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn take_js_value_by_value(x: JsValue) {}

#[wasm_bindgen]
pub fn take_js_value_by_shared_ref(x: &JsValue) {}

#[wasm_bindgen]
pub fn return_js_value() -> JsValue {
    JsValue::NULL
}
