use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;
use js_sys::*;

#[wasm_bindgen_test]
fn new_undefined() {
    assert_eq!(Boolean::new(JsValue::undefined()).value_of(), false);
}

#[wasm_bindgen_test]
fn new_truely() {
    assert_eq!(Boolean::new(JsValue::from("foo")).value_of(), true);
}
