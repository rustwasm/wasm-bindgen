use js_sys::Object;
use wasm_bindgen_test::*;

include!(concat!(env!("OUT_DIR"), "/global.rs"));

#[wasm_bindgen_test]
fn works() {
    assert_eq!(Global::global_no_args(), 3);
    assert_eq!(Global::global_with_args("a", "b"), "ab");
    assert_eq!(Global::global_attribute(), "x");
    Global::set_global_attribute("y");
    assert_eq!(Global::global_attribute(), "y");
}
