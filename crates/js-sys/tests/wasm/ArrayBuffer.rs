use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;
use js_sys::*;

#[wasm_bindgen_test]
fn new() {
    let x = ArrayBuffer::new(42);
    let y: JsValue = x.into();
    assert!(y.is_object());
}

#[wasm_bindgen_test]
fn is_view() {
    let x = Uint8Array::new(JsValue::from(42));
    assert!(ArrayBuffer::is_view(JsValue::from(x)));
}

#[test]
fn slice() {
    let buf = ArrayBuffer::new(4);
    let slice = buf.slice(2);
    assert!(JsValue::from(slice).is_object());
}

#[test]
fn slice_with_end() {
    let buf = ArrayBuffer::new(4);
    let slice = buf.slice_with_end(1, 2);
    assert!(JsValue::from(slice).is_object());
}
