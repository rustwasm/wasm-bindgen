use super::project;

#[test]
fn simple() {
    project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros, wasm_import_module)]

                extern crate wasm_bindgen;

                use wasm_bindgen::prelude::*;

                #[wasm_bindgen(module = "./test")]
                extern {
                    fn foo(s: &JsValue);
                }

                #[wasm_bindgen]
                pub fn bar(s: &JsValue) {
                    foo(s);
                }
            "#,
        )
        .file(
            "test.js",
            r#"
                import * as wasm from "./out";
                import * as assert from "assert";

                let ARG = null;

                export function foo(s) {
                    assert.strictEqual(ARG, null);
                    ARG = s;
                }

                export function test() {
                    assert.strictEqual(ARG, null);
                    let sym = Symbol('test');
                    wasm.bar(sym);
                    assert.strictEqual(ARG, sym);
                }
            "#,
        )
        .test();
}

#[test]
fn owned() {
    project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros, wasm_import_module)]

                extern crate wasm_bindgen;

                use wasm_bindgen::prelude::*;

                #[wasm_bindgen(module = "./test")]
                extern {
                    fn foo(s: JsValue);
                }

                #[wasm_bindgen]
                pub fn bar(s: JsValue) {
                    foo(s);
                }
            "#,
        )
        .file(
            "test.js",
            r#"
                import * as wasm from "./out";
                import * as assert from "assert";

                let ARG = null;

                export function foo(s) {
                    assert.strictEqual(ARG, null);
                    ARG = s;
                }

                export function test() {
                    assert.strictEqual(ARG, null);
                    let sym = Symbol('test');
                    wasm.bar(sym);
                    assert.strictEqual(ARG, sym);
                }
            "#,
        )
        .test();
}

#[test]
fn clone() {
    project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros, wasm_import_module)]

                extern crate wasm_bindgen;

                use wasm_bindgen::prelude::*;

                #[wasm_bindgen(module = "./test")]
                extern {
                    fn foo1(s: JsValue);
                    fn foo2(s: &JsValue);
                    fn foo3(s: JsValue);
                    fn foo4(s: &JsValue);
                    fn foo5(s: JsValue);
                }

                #[wasm_bindgen]
                pub fn bar(s: JsValue) {
                    foo1(s.clone());
                    foo2(&s);
                    foo3(s.clone());
                    foo4(&s);
                    foo5(s);
                }
            "#,
        )
        .file(
            "test.js",
            r#"
                import * as wasm from "./out";
                import * as assert from "assert";

                let ARG = Symbol('test');

                export function foo1(s) { assert.strictEqual(s, ARG); }
                export function foo2(s) { assert.strictEqual(s, ARG); }
                export function foo3(s) { assert.strictEqual(s, ARG); }
                export function foo4(s) { assert.strictEqual(s, ARG); }
                export function foo5(s) { assert.strictEqual(s, ARG); }

                export function test() {
                    wasm.bar(ARG);
                }
            "#,
        )
        .test();
}

#[test]
fn promote() {
    project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros, wasm_import_module)]

                extern crate wasm_bindgen;

                use wasm_bindgen::prelude::*;

                #[wasm_bindgen(module = "./test")]
                extern {
                    fn foo1(s: &JsValue);
                    fn foo2(s: JsValue);
                    fn foo3(s: &JsValue);
                    fn foo4(s: JsValue);
                }

                #[wasm_bindgen]
                pub fn bar(s: &JsValue) {
                    foo1(s);
                    foo2(s.clone());
                    foo3(s);
                    foo4(s.clone());
                }
            "#,
        )
        .file(
            "test.js",
            r#"
                import * as wasm from "./out";
                import * as assert from "assert";

                let ARG = Symbol('test');

                export function foo1(s) { assert.strictEqual(s, ARG); }
                export function foo2(s) { assert.strictEqual(s, ARG); }
                export function foo3(s) { assert.strictEqual(s, ARG); }
                export function foo4(s) { assert.strictEqual(s, ARG); }

                export function test() {
                    wasm.bar(ARG);
                }
            "#,
        )
        .test();
}

#[test]
fn returning_vector() {
    project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros, wasm_import_module)]

                extern crate wasm_bindgen;

                use wasm_bindgen::prelude::*;

                #[wasm_bindgen(module = "./test")]
                extern {
                    fn foo() -> JsValue;
                }

                #[wasm_bindgen]
                pub fn bar() -> Vec<JsValue> {
                    let mut res = Vec::new();
                    for _ in 0..10 {
                        res.push(foo())
                    }
                    res
                }
            "#,
        )
        .file(
            "test.js",
            r#"
                import * as wasm from "./out";
                import * as assert from "assert";

                export function foo() { return { "foo": "bar" }; }

                export function test() {
                    const result = wasm.bar();
                    assert.strictEqual(result.length, 10);
                }
            "#,
        )
        .test();
}

#[test]
fn another_vector_return() {
    project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros, wasm_import_module)]

                extern crate wasm_bindgen;

                use wasm_bindgen::prelude::*;


                #[wasm_bindgen]
                pub fn get_array() -> Vec<JsValue> {
                    vec![
                        JsValue::from(1),
                        JsValue::from(2),
                        JsValue::from(3),
                        JsValue::from(4),
                        JsValue::from(5),
                        JsValue::from(6),
                    ]
                }
            "#,
        )
        .file(
            "test.js",
            r#"
                import { get_array } from "./out";
                import * as assert from "assert";

                export function test() {
                    assert.deepStrictEqual(get_array(), [1, 2, 3, 4, 5, 6]);
                }
            "#,
        )
        .test();
}

#[test]
fn serde() {
    project()
        .serde(true)
        .depend("serde = '1.0'")
        .depend("serde_derive = '1.0'")
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros, wasm_import_module)]

                extern crate wasm_bindgen;
                #[macro_use]
                extern crate serde_derive;

                use wasm_bindgen::prelude::*;

                #[derive(Deserialize, Serialize)]
                pub struct Foo {
                    a: u32,
                    b: String,
                    c: Option<Bar>,
                    d: Bar,
                }

                #[derive(Deserialize, Serialize)]
                pub struct Bar {
                    a: u32,
                }

                #[wasm_bindgen(module = "./test")]
                extern {
                    fn verify(a: JsValue) -> JsValue;
                }

                #[wasm_bindgen]
                pub fn run() {
                    let js = JsValue::from_serde("foo").unwrap();
                    assert_eq!(js.as_string(), Some("foo".to_string()));

                    let ret = verify(JsValue::from_serde(&Foo {
                        a: 0,
                        b: "foo".to_string(),
                        c: None,
                        d: Bar { a: 1 },
                    }).unwrap());

                    let foo = ret.into_serde::<Foo>().unwrap();
                    assert_eq!(foo.a, 2);
                    assert_eq!(foo.b, "bar");
                    assert!(foo.c.is_some());
                    assert_eq!(foo.c.as_ref().unwrap().a, 3);
                    assert_eq!(foo.d.a, 4);
                }

                #[wasm_bindgen]
                pub fn parse(j: &JsValue) {
                    let s = j.into_serde::<String>().unwrap();
                    assert_eq!(s, "bar");
                }
            "#,
        )
        .file(
            "test.js",
            r#"
                import { run, parse } from "./out";
                import * as assert from "assert";

                export function verify(a) {
                    assert.deepStrictEqual(a, {
                        a: 0,
                        b: 'foo',
                        c: null,
                        d: { a: 1 }
                    });

                    return {
                        a: 2,
                        b: 'bar',
                        c: { a: 3 },
                        d: { a: 4 },
                    }
                }

                export function test() {
                    run();
                    parse('bar');
                }
            "#,
        )
        .test();
}
