// Keep these tests in alphabetical order, just like the imports in `src/js.rs`.

use super::project;

mod Array;
mod ArrayBuffer;
mod ArrayIterator;
mod Boolean;
mod DataView;
mod Date;
mod Error;
mod Function;
mod Generator;
mod Intl;
mod JsString;
mod Map;
mod MapIterator;
mod Math;
mod Number;
mod Object;
mod Proxy;
mod Reflect;
mod Set;
mod SetIterator;
mod Symbol;
mod TypedArray;
mod WeakMap;
mod WeakSet;
mod WebAssembly;

#[test]
#[cfg(feature = "std")]
fn decode_uri() {
    project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros)]

                extern crate wasm_bindgen;
                use wasm_bindgen::prelude::*;
                use wasm_bindgen::js;

                #[wasm_bindgen]
                pub fn test() {
                    let x = js::decode_uri("https://mozilla.org/?x=%D1%88%D0%B5%D0%BB%D0%BB%D1%8B")
                        .ok()
                        .expect("should decode URI OK");
                    assert_eq!(String::from(x), "https://mozilla.org/?x=шеллы");

                    assert!(js::decode_uri("%E0%A4%A").is_err());
                }
            "#,
        )
        .test();
}

#[test]
fn decode_uri_component() {
    project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros)]

                extern crate wasm_bindgen;
                use wasm_bindgen::prelude::*;
                use wasm_bindgen::js;

                #[wasm_bindgen]
                pub fn test() {
                    let x = js::decode_uri_component("%3Fx%3Dtest")
                        .ok()
                        .expect("should decode URI OK");
                    assert_eq!(String::from(x), "?x=test");

                    assert!(js::decode_uri_component("%E0%A4%A").is_err());
                }
            "#,
        )
        .test();
}

#[test]
#[cfg(feature = "std")]
fn encode_uri() {
    project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros)]

                extern crate wasm_bindgen;
                use wasm_bindgen::prelude::*;
                use wasm_bindgen::js;

                #[wasm_bindgen]
                pub fn test() {
                    let x = js::encode_uri("ABC abc 123");
                    assert_eq!(String::from(x), "ABC%20abc%20123");
                }
            "#,
        )
        .test();
}

#[test]
fn encode_uri_component() {
    project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros)]

                extern crate wasm_bindgen;
                use wasm_bindgen::prelude::*;
                use wasm_bindgen::js;

                #[wasm_bindgen]
                pub fn test() {
                    let x = js::encode_uri_component("?x=шеллы");
                    assert_eq!(String::from(x), "%3Fx%3D%D1%88%D0%B5%D0%BB%D0%BB%D1%8B");
                }
            "#,
        )
        .test();
}

#[test]
fn eval() {
    project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros)]

                extern crate wasm_bindgen;
                use wasm_bindgen::prelude::*;
                use wasm_bindgen::js;

                #[wasm_bindgen]
                pub fn test() {
                    let x = js::eval("42").ok().expect("should eval OK");
                    assert_eq!(x.as_f64().unwrap(), 42.0);

                    let err = js::eval("(function () { throw 42; }())")
                        .err()
                        .expect("eval should throw");
                    assert_eq!(err.as_f64().unwrap(), 42.0);
                }
            "#,
        )
        .test();
}

#[test]
fn is_finite() {
    project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros)]

                extern crate wasm_bindgen;
                use wasm_bindgen::prelude::*;
                use wasm_bindgen::js;

                #[wasm_bindgen]
                pub fn is_finite(value: &JsValue) -> bool {
                    js::is_finite(value)
                }
            "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.is_finite(42), true);
                assert.equal(wasm.is_finite(42.1), true);
                assert.equal(wasm.is_finite('42'), true);
                assert.equal(wasm.is_finite(NaN), false);
                assert.equal(wasm.is_finite(Infinity), false);
            }
        "#,
        )
        .test();
}

#[test]
fn parse_int_float() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn test() {
                let i = js::parse_int("42", 10);
                assert_eq!(i as i64, 42);

                let i = js::parse_int("42", 16);
                assert_eq!(i as i64, 66); // 0x42 == 66

                let i = js::parse_int("invalid int", 10);
                assert!(i.is_nan());

                let f = js::parse_float("123456.789");
                assert_eq!(f, 123456.789);

                let f = js::parse_float("invalid float");
                assert!(f.is_nan());
            }
        "#)
        .test();
}

#[test]
fn escape() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn test() {
                assert_eq!(String::from(js::escape("test")), "test");
                assert_eq!(String::from(js::escape("äöü")), "%E4%F6%FC");
                assert_eq!(String::from(js::escape("ć")), "%u0107");
                assert_eq!(String::from(js::escape("@*_+-./")), "@*_+-./");
            }
        "#)
        .test();
}