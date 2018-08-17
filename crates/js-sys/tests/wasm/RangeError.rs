use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;
use wasm_bindgen::JsCast;
use js_sys::*;

#[wasm_bindgen_test]
fn range_error() {
    let error = RangeError::new("out of range yo");
    assert!(error.is_instance_of::<RangeError>());
    assert!(error.is_instance_of::<Error>());
    assert!(error.is_instance_of::<Object>());

    let base: &Error = error.as_ref();
    assert_eq!(JsValue::from(base.message()), "out of range yo");
}
