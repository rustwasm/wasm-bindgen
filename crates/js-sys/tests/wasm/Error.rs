use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;
use js_sys::*;

#[wasm_bindgen_test]
fn new() {
    let error = Error::new(&"some message".into());
    assert_eq!(JsValue::from(error.message()), "some message");
}

#[wasm_bindgen_test]
fn set_message() {
    let error = Error::new(&"test".into());
    error.set_message(&"another".into());
    assert_eq!(JsValue::from(error.message()), "another");
}

#[wasm_bindgen_test]
fn name() {
    let error = Error::new(&"test".into());
    assert_eq!(JsValue::from(error.name()), "Error");
}

#[wasm_bindgen_test]
fn set_name() {
    let error = Error::new(&"test".into());
    error.set_name(&"different".into());
    assert_eq!(JsValue::from(error.name()), "different");
}

#[wasm_bindgen_test]
fn to_string() {
    let error = Error::new(&"error message 1".into());
    assert_eq!(JsValue::from(error.to_string()), "Error: error message 1");
    error.set_name(&"error_name_1".into());
    assert_eq!(JsValue::from(error.to_string()), "error_name_1: error message 1");
}
