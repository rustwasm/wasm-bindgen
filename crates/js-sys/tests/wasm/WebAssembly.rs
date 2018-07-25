use wasm_bindgen_test::*;
use js_sys::*;

#[wasm_bindgen_test]
fn validate() {
    assert!(!WebAssembly::validate(&ArrayBuffer::new(42).into()).unwrap());

    assert!(WebAssembly::validate(&2.into()).is_err());
}
