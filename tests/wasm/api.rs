use wasm_bindgen_test::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "tests/wasm/api.js", version = "*")]
extern {
    fn test_works();
    fn test_eq_works();
    fn assert_null(v: JsValue);
}

#[wasm_bindgen_test]
fn works() {
    test_works();
}

#[wasm_bindgen]
pub fn api_foo() -> JsValue {
    JsValue::from("foo")
}

#[wasm_bindgen]
pub fn api_bar(s: &str) -> JsValue {
    JsValue::from(s)
}

#[wasm_bindgen]
pub fn api_baz() -> JsValue {
    JsValue::from(1.0)
}

#[wasm_bindgen]
pub fn api_baz2(a: &JsValue, b: &JsValue) {
    assert_eq!(a.as_f64(), Some(2.0));
    assert_eq!(b.as_f64(), None);
}

#[wasm_bindgen]
pub fn api_js_null() -> JsValue {
    JsValue::null()
}

#[wasm_bindgen]
pub fn api_js_undefined() -> JsValue {
    JsValue::undefined()
}

#[wasm_bindgen]
pub fn api_test_is_null_undefined(
    a: &JsValue,
    b: &JsValue,
    c: &JsValue,
) {
    assert!(a.is_null());
    assert!(!a.is_undefined());

    assert!(!b.is_null());
    assert!(b.is_undefined());

    assert!(!c.is_null());
    assert!(!c.is_undefined());
}

#[wasm_bindgen]
pub fn api_get_true() -> JsValue {
    JsValue::from(true)
}

#[wasm_bindgen]
pub fn api_get_false() -> JsValue {
    JsValue::from(false)
}

#[wasm_bindgen]
pub fn api_test_bool(
    a: &JsValue,
    b: &JsValue,
    c: &JsValue,
) {
    assert_eq!(a.as_bool(), Some(true));
    assert_eq!(format!("{:?}", a), "true");
    assert_eq!(b.as_bool(), Some(false));
    assert_eq!(c.as_bool(), None);
}

#[wasm_bindgen]
pub fn api_mk_symbol() -> JsValue {
    let a = JsValue::symbol(None);
    assert!(a.is_symbol());
    assert_eq!(format!("{:?}", a), "Symbol(..)");
    return a
}

#[wasm_bindgen]
pub fn api_mk_symbol2(s: &str) -> JsValue {
    let a = JsValue::symbol(Some(s));
    assert!(a.is_symbol());
    return a
}

#[wasm_bindgen]
pub fn api_assert_symbols(a: &JsValue, b: &JsValue) {
    assert!(a.is_symbol());
    assert!(!b.is_symbol());
}

#[wasm_bindgen]
pub fn api_acquire_string(a: &JsValue, b: &JsValue) {
    assert_eq!(a.as_string().unwrap(), "foo");
    assert_eq!(format!("{:?}", a), "\"foo\"");
    assert_eq!(b.as_string(), None);
}

#[wasm_bindgen]
pub fn api_acquire_string2(a: &JsValue) -> String {
    a.as_string().unwrap_or("wrong".to_string())
}

#[wasm_bindgen_test]
fn eq_works() {
    test_eq_works();
}

#[wasm_bindgen]
pub fn eq_test(a: &JsValue, b: &JsValue) -> bool {
    a == b
}

#[wasm_bindgen]
pub fn eq_test1(a: &JsValue) -> bool {
    a == a
}

#[wasm_bindgen_test]
fn null_keeps_working() {
    assert_null(JsValue::null());
    assert_null(JsValue::null());
}
