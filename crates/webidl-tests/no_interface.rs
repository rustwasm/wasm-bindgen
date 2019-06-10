use wasm_bindgen_test::*;

include!(concat!(env!("OUT_DIR"), "/no_interface.rs"));

#[wasm_bindgen_test]
fn smoke() {
    let obj = GetNoInterfaceObject::get();
    assert_eq!(obj.number(), 3.0);
    obj.foo();
}
