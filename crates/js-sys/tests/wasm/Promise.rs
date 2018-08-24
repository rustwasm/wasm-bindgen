use wasm_bindgen_test::*;
use wasm_bindgen::JsCast;
use js_sys::*;

#[wasm_bindgen_test]
fn promise_inheritance() {
    let promise = Promise::new(&mut |_, _| ());
    assert!(promise.is_instance_of::<Promise>());
    assert!(promise.is_instance_of::<Object>());
    let _: &Object = promise.as_ref();
}
