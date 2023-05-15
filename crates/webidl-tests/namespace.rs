use crate::generated::*;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn simple_namespace_test() {
    assert_eq!(math_test::add_one(1), 2);
    assert_eq!(math_test::pow(1.0, 100.0), 1.0);
    assert_eq!(math_test::pow(10.0, 2.0), 100.0);
}
