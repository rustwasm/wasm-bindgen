use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;
use wasm_bindgen::JsCast;
use js_sys::*;

#[wasm_bindgen_test]
fn reference_error() {
    let error = ReferenceError::new("bad reference, fool");
    assert!(error.is_instance_of::<ReferenceError>());
    assert!(error.is_instance_of::<Error>());
    assert!(error.is_instance_of::<Object>());

    let base: &Error = error.as_ref();
    assert_eq!(JsValue::from(base.message()), "bad reference, fool");
}
