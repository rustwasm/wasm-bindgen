use wasm_bindgen_test::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "tests/wasm/structural.js")]
extern {
    fn js_works();
}

#[wasm_bindgen]
extern {
    pub type Foo;

    #[wasm_bindgen(method, structural)]
    fn bar(this: &Foo);
    #[wasm_bindgen(method, getter, structural)]
    fn baz(this: &Foo) -> u32;
    #[wasm_bindgen(method, setter, structural)]
    fn set_baz(this: &Foo, val: u32);
}

#[wasm_bindgen]
pub fn run(a: &Foo) {
    a.bar();
    assert_eq!(a.baz(), 1);
    a.set_baz(2);
    assert_eq!(a.baz(), 2);
}

#[wasm_bindgen_test]
fn works() {
    js_works();
}
