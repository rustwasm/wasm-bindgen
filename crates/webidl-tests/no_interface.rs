use wasm_bindgen_test::*;
use crate::generated::*;

#[wasm_bindgen_test]
fn smoke() {
    let obj = GetNoInterfaceObject::get();
    assert_eq!(obj.number(), 3.0);
    obj.foo();
}
