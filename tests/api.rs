extern crate test_support;

#[test]
fn works() {
    test_support::project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            wasm_bindgen! {
                pub fn foo() -> JsObject {
                    JsObject::from("foo")
                }

                pub fn bar(s: &str) -> JsObject {
                    JsObject::from(s)
                }

                pub fn baz() -> JsObject {
                    JsObject::from(1.0)
                }

                pub fn baz2(a: &JsObject, b: &JsObject) {
                    assert_eq!(a.as_f64(), Some(2.0));
                    assert_eq!(b.as_f64(), None);
                }

                pub fn js_null() -> JsObject {
                    JsObject::null()
                }

                pub fn js_undefined() -> JsObject {
                    JsObject::undefined()
                }

                pub fn test_is_null_undefined(
                    a: &JsObject,
                    b: &JsObject,
                    c: &JsObject,
                ) {
                    assert!(a.is_null());
                    assert!(!a.is_undefined());

                    assert!(!b.is_null());
                    assert!(b.is_undefined());

                    assert!(!c.is_null());
                    assert!(!c.is_undefined());
                }

                pub fn get_true() -> JsObject {
                    JsObject::from(true)
                }

                pub fn get_false() -> JsObject {
                    JsObject::from(false)
                }

                pub fn test_bool(
                    a: &JsObject,
                    b: &JsObject,
                    c: &JsObject,
                ) {
                    assert_eq!(a.as_bool(), Some(true));
                    assert_eq!(b.as_bool(), Some(false));
                    assert_eq!(c.as_bool(), None);
                }

                pub fn mk_symbol() -> JsObject {
                    let a = JsObject::symbol(None);
                    assert!(a.is_symbol());
                    return a
                }

                pub fn mk_symbol2(s: &str) -> JsObject {
                    let a = JsObject::symbol(Some(s));
                    assert!(a.is_symbol());
                    return a
                }

                pub fn assert_symbols(a: &JsObject, b: &JsObject) {
                    assert!(a.is_symbol());
                    assert!(!b.is_symbol());
                }

                pub fn acquire_string(a: &JsObject, b: &JsObject) {
                    assert_eq!(a.as_string().unwrap(), "foo");
                    assert_eq!(b.as_string(), None);
                }

                pub fn acquire_string2(a: &JsObject) -> String {
                    a.as_string().unwrap_or("wrong".to_string())
                }
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import { Exports, Imports } from "./out";

            export const imports: Imports = {};

            export function test(wasm: Exports) {
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
        "#)
        .test();
}

