use super::project;

#[test]
fn works() {
    project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros)]

                extern crate wasm_bindgen;

                use wasm_bindgen::prelude::*;

                #[wasm_bindgen]
                pub fn foo() -> JsValue {
                    JsValue::from("foo")
                }

                #[wasm_bindgen]
                pub fn bar(s: &str) -> JsValue {
                    JsValue::from(s)
                }

                #[wasm_bindgen]
                pub fn baz() -> JsValue {
                    JsValue::from(1.0)
                }

                #[wasm_bindgen]
                pub fn baz2(a: &JsValue, b: &JsValue) {
                    assert_eq!(a.as_f64(), Some(2.0));
                    assert_eq!(b.as_f64(), None);
                }

                #[wasm_bindgen]
                pub fn js_null() -> JsValue {
                    JsValue::null()
                }

                #[wasm_bindgen]
                pub fn js_undefined() -> JsValue {
                    JsValue::undefined()
                }

                #[wasm_bindgen]
                pub fn test_is_null_undefined(
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
                pub fn get_true() -> JsValue {
                    JsValue::from(true)
                }

                #[wasm_bindgen]
                pub fn get_false() -> JsValue {
                    JsValue::from(false)
                }

                #[wasm_bindgen]
                pub fn test_bool(
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
                pub fn mk_symbol() -> JsValue {
                    let a = JsValue::symbol(None);
                    assert!(a.is_symbol());
                    assert_eq!(format!("{:?}", a), "Symbol(..)");
                    return a
                }

                #[wasm_bindgen]
                pub fn mk_symbol2(s: &str) -> JsValue {
                    let a = JsValue::symbol(Some(s));
                    assert!(a.is_symbol());
                    return a
                }

                #[wasm_bindgen]
                pub fn assert_symbols(a: &JsValue, b: &JsValue) {
                    assert!(a.is_symbol());
                    assert!(!b.is_symbol());
                }

                #[wasm_bindgen]
                pub fn acquire_string(a: &JsValue, b: &JsValue) {
                    assert_eq!(a.as_string().unwrap(), "foo");
                    assert_eq!(format!("{:?}", a), "\"foo\"");
                    assert_eq!(b.as_string(), None);
                }

                #[wasm_bindgen]
                pub fn acquire_string2(a: &JsValue) -> String {
                    a.as_string().unwrap_or("wrong".to_string())
                }
            "#,
        )
        .file(
            "test.js",
            r#"
                import * as assert from "assert";
                import * as wasm from "./out";

                export function test() {
                    assert.strictEqual(wasm.foo(), 'foo');
                    assert.strictEqual(wasm.bar('a'), 'a');
                    assert.strictEqual(wasm.baz(), 1);
                    wasm.baz2(2, 'a');

                    assert.strictEqual(wasm.js_null(), null);
                    assert.strictEqual(wasm.js_undefined(), undefined);

                    wasm.test_is_null_undefined(null, undefined, 1.0);

                    assert.strictEqual(wasm.get_true(), true);
                    assert.strictEqual(wasm.get_false(), false);
                    wasm.test_bool(true, false, 1.0);

                    assert.strictEqual(typeof(wasm.mk_symbol()), 'symbol');
                    assert.strictEqual(typeof(wasm.mk_symbol2('a')), 'symbol');
                    assert.strictEqual(Symbol.keyFor(wasm.mk_symbol()), undefined);
                    assert.strictEqual(Symbol.keyFor(wasm.mk_symbol2('b')), undefined);

                    wasm.assert_symbols(Symbol(), 'a');
                    wasm.acquire_string('foo', null)
                    assert.strictEqual(wasm.acquire_string2(''), '');
                    assert.strictEqual(wasm.acquire_string2('a'), 'a');
                }
            "#,
        )
        .test();
}

#[test]
fn eq_works() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            pub fn test(a: &JsValue, b: &JsValue) -> bool {
                a == b
            }

            #[wasm_bindgen]
            pub fn test1(a: &JsValue) -> bool {
                a == a
            }
        "#,
        )
        .file(
            "test.js",
            r#"
                import * as assert from "assert";
                import * as wasm from "./out";

                export function test() {
                    assert.strictEqual(wasm.test('a', 'a'), true);
                    assert.strictEqual(wasm.test('a', 'b'), false);
                    assert.strictEqual(wasm.test(NaN, NaN), false);
                    assert.strictEqual(wasm.test({a: 'a'}, {a: 'a'}), false);
                    assert.strictEqual(wasm.test1(NaN), false);
                    let x = {a: 'a'};
                    assert.strictEqual(wasm.test(x, x), true);
                    assert.strictEqual(wasm.test1(x), true);
                }
            "#,
        )
        .test();
}
