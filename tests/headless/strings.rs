use js_sys::JsString;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "/tests/headless/strings.js")]
extern "C" {
    fn test_string_roundtrip(c: &Closure<dyn Fn(String) -> String>);

    fn identity(s: &str) -> String;
}

#[wasm_bindgen_test]
fn string_roundtrip() {
    test_string_roundtrip(&Closure::wrap(Box::new(|s| s)));

    assert_eq!("\u{feff}bar", &identity("\u{feff}bar"));

    assert_eq!(STRING.with(|s| String::from(s)), "foo");
}

#[wasm_bindgen]
// Currently Rustfmt simply removes the value on this static.
// See <https://github.com/rust-lang/rustfmt/issues/6267>.
#[rustfmt::skip]
extern "C" {
    #[wasm_bindgen(thread_local_v2, static_string)]
    static STRING: JsString = "foo";
}
