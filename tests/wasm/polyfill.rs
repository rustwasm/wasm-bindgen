use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/polyfill.js")]
extern "C" {
    fn import_me();
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(polyfill = PolyfillBar)]
    type PolyfillFoo;
    #[wasm_bindgen(constructor)]
    fn new() -> PolyfillFoo;
    #[wasm_bindgen(method)]
    fn foo(this: &PolyfillFoo) -> u32;

    #[wasm_bindgen(polyfill = PolyfillBaz1, polyfill = PolyfillBar)]
    type PolyfillFoo2;
    #[wasm_bindgen(constructor)]
    fn new() -> PolyfillFoo2;
    #[wasm_bindgen(method)]
    fn foo(this: &PolyfillFoo2) -> u32;

    #[wasm_bindgen(polyfill = PolyfillBaz1, polyfill = PolyfillBar, polyfill = PolyfillBaz2)]
    type PolyfillFoo3;
    #[wasm_bindgen(constructor)]
    fn new() -> PolyfillFoo3;
    #[wasm_bindgen(method)]
    fn foo(this: &PolyfillFoo3) -> u32;
}

#[wasm_bindgen_test]
pub fn polyfill_works() {
    import_me();

    assert_eq!(PolyfillFoo::new().foo(), 123);
    assert_eq!(PolyfillFoo2::new().foo(), 123);
    assert_eq!(PolyfillFoo3::new().foo(), 123);
}
