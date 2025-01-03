use js_sys::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;

#[allow(deprecated)]
#[wasm_bindgen_test]
fn new_undefined() {
    assert!(!Boolean::new(&JsValue::undefined()).value_of());
}

#[allow(deprecated)]
#[wasm_bindgen_test]
fn new_truly() {
    assert!(Boolean::new(&JsValue::from("foo")).value_of());
}

#[allow(deprecated)]
#[wasm_bindgen_test]
fn boolean_inheritance() {
    let b = Boolean::new(&JsValue::from(true));
    assert!(b.is_instance_of::<Boolean>());
    assert!(b.is_instance_of::<Object>());
    let _: &Object = b.as_ref();
}
