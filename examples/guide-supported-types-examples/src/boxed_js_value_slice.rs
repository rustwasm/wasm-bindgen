use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn take_boxed_js_value_slice_by_value(x: Box<[JsValue]>) {}

#[wasm_bindgen]
pub fn return_boxed_js_value_slice() -> Box<[JsValue]> {
    vec![JsValue::NULL, JsValue::UNDEFINED].into_boxed_slice()
}

#[wasm_bindgen]
pub fn take_option_boxed_js_value_slice(x: Option<Box<[JsValue]>>) {}

#[wasm_bindgen]
pub fn return_option_boxed_js_value_slice() -> Option<Box<[JsValue]>> {
    None
}
