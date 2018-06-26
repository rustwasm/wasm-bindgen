// Keep these tests in alphabetical order, just like the imports in `src/js.rs`.

use super::project;

mod Object;
mod Array;
mod ArrayIterator;
mod JsFunction;
mod JsString;
mod Number;
mod Math;
mod TypedArray;

#[test]
#[cfg(feature = "std")]
fn decode_uri() {
    project()
        .file("src/lib.rs", r#"
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
        "#)
        .test();
}

#[test]
#[cfg(feature = "std")]
fn encode_uri() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn test() {
                let x = js::encode_uri("ABC abc 123");
                assert_eq!(String::from(x), "ABC%20abc%20123");
            }
        "#)
        .test();
}

#[test]
fn eval() {
    project()
        .file("src/lib.rs", r#"
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
        "#)
        .test();
}
