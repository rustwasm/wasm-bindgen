use super::project;

#[test]
fn serde() {
    project()
        .serde(true)
        .depend("serde = '1.0'")
        .depend("serde_derive = '1.0'")
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros)]

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
