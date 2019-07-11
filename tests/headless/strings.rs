use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "/tests/headless/strings.js")]
extern "C" {
    fn test_string_roundtrip(c: &Closure<dyn Fn(String) -> String>);
}

#[wasm_bindgen_test]
fn string_roundtrip() {
    test_string_roundtrip(&Closure::wrap(Box::new(|s| s)));
}
