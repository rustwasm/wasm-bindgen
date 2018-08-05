use wasm_bindgen_test::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "tests/wasm/js_objects.js", version = "*")]
extern {
    fn simple_foo(s: &JsValue);
    fn js_simple();

    fn owned_foo(s: JsValue);
    fn js_owned();

    fn clone_foo1(s: JsValue);
    fn clone_foo2(s: &JsValue);
    fn clone_foo3(s: JsValue);
    fn clone_foo4(s: &JsValue);
    fn clone_foo5(s: JsValue);
    fn js_clone();

    fn promote_foo1(s: &JsValue);
    fn promote_foo2(s: JsValue);
    fn promote_foo3(s: &JsValue);
    fn promote_foo4(s: JsValue);
    fn js_promote();

    fn returning_vector_foo() -> JsValue;
    fn js_returning_vector();

    fn js_another_vector_return();
}

#[wasm_bindgen]
pub fn simple_bar(s: &JsValue) {
    simple_foo(s);
}

#[wasm_bindgen_test]
fn simple() {
    js_simple();
}

#[wasm_bindgen]
pub fn owned_bar(s: JsValue) {
    owned_foo(s);
}

#[wasm_bindgen_test]
fn owned() {
    js_owned();
}

#[wasm_bindgen]
pub fn clone_bar(s: JsValue) {
    clone_foo1(s.clone());
    clone_foo2(&s);
    clone_foo3(s.clone());
    clone_foo4(&s);
    clone_foo5(s);
}

#[wasm_bindgen_test]
fn clone() {
    js_clone();
}

#[wasm_bindgen]
pub fn promote_bar(s: &JsValue) {
    promote_foo1(s);
    promote_foo2(s.clone());
    promote_foo3(s);
    promote_foo4(s.clone());
}

#[wasm_bindgen_test]
fn promote() {
    js_promote();
}

#[wasm_bindgen]
pub fn returning_vector_bar() -> Vec<JsValue> {
    let mut res = Vec::new();
    for _ in 0..10 {
        res.push(returning_vector_foo())
    }
    res
}

#[wasm_bindgen_test]
fn returning_vector() {
    js_returning_vector();
}

#[wasm_bindgen]
pub fn another_vector_return_get_array() -> Vec<JsValue> {
    vec![
        JsValue::from(1),
        JsValue::from(2),
        JsValue::from(3),
        JsValue::from(4),
        JsValue::from(5),
        JsValue::from(6),
    ]
}

#[wasm_bindgen_test]
fn another_vector_return() {
    js_another_vector_return();
}
