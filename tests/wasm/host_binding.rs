use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen]
extern "C" {
    type Math;
    #[wasm_bindgen(static_method_of = Math, host_binding)]
    fn log(f: f32) -> f32;
}

#[wasm_bindgen(module = "tests/wasm/host_binding.js")]
extern "C" {
    type MyType;
    #[wasm_bindgen(constructor, host_binding)]
    fn new(x: u32) -> MyType;
    #[wasm_bindgen(static_method_of = MyType, host_binding)]
    fn foo(a: &str) -> String;
    #[wasm_bindgen(method, host_binding)]
    fn bar(this: &MyType, arg: bool) -> f32;

    #[wasm_bindgen(method, getter, host_binding)]
    fn a(this: &MyType) -> u32;
    #[wasm_bindgen(method, setter, host_binding)]
    fn set_a(this: &MyType, a: u32);
}

#[wasm_bindgen_test]
fn simple() {
    assert_eq!(Math::log(1.0), 0.0);
}

#[wasm_bindgen_test]
fn classes() {
    assert_eq!(MyType::foo("x"), "xy");
    let x = MyType::new(2);
    assert_eq!(x.bar(true), 3.2);
    assert_eq!(x.a(), 1);
    x.set_a(3);
    assert_eq!(x.a(), 3);
}
