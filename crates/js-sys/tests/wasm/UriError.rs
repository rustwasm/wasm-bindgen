use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;
use wasm_bindgen::JsCast;
use js_sys::*;

#[wasm_bindgen_test]
fn uri_error() {
    let error = UriError::new("msg");
    assert!(error.is_instance_of::<UriError>());
    assert!(error.is_instance_of::<Error>());
    assert!(error.is_instance_of::<Object>());

    let base: &Error = error.as_ref();
    assert_eq!(JsValue::from(base.message()), "msg");
}
