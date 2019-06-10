use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

include!(concat!(env!("OUT_DIR"), "/global.rs"));

#[wasm_bindgen]
extern "C" {
    fn get_global() -> Global;
}

#[wasm_bindgen_test]
fn works() {
    let x = get_global();
    assert_eq!(x.global_no_args(), 3);
    assert_eq!(x.global_with_args("a", "b"), "ab");
    assert_eq!(x.global_attribute(), "x");
    x.set_global_attribute("y");
    assert_eq!(x.global_attribute(), "y");
}
