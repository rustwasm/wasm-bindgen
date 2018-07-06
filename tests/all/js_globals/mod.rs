// Keep these tests in alphabetical order, just like the imports in `src/js.rs`.

use super::project;

mod Array;
mod ArrayBuffer;
mod ArrayIterator;
mod Boolean;
mod Date;
mod Error;
mod Function;
mod Generator;
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

#[test]
#[cfg(feature = "std")]
fn decode_uri() {
    project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(proc_macro, wasm_custom_section)]

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
                #![feature(proc_macro, wasm_custom_section)]

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
                #![feature(proc_macro, wasm_custom_section)]

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
                #![feature(proc_macro, wasm_custom_section)]

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
                #![feature(proc_macro, wasm_custom_section)]

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
