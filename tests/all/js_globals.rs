// Keep these tests in alphabetical order, just like the imports in `src/js.rs`.

use super::project;

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
                assert_eq!(x, "https://mozilla.org/?x=шеллы");

                assert!(js::decode_uri("%E0%A4%A").is_err());
            }
        "#)
        .file("test.ts", r#"
            import * as wasm from "./out";

            export function test() {
                wasm.test();
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
                assert_eq!(x, "ABC%20abc%20123");
            }
        "#)
        .file("test.ts", r#"
            import * as wasm from "./out";

            export function test() {
                wasm.test();
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
        .file("test.ts", r#"
            import * as wasm from "./out";

            export function test() {
                wasm.test();
            }
        "#)
        .test();
}

#[allow(non_snake_case)]
mod Object {
    use project;

    #[test]
    fn new() {
        project()
            .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn new_object() -> js::Object {
                js::Object::new()
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(typeof wasm.new_object(), "object");
            }
        "#)
        .test()
    }

    #[test]
    fn has_own_property() {
        project()
            .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn has_own_foo_property(obj: &js::Object) -> bool {
                obj.has_own_property("foo")
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.ok(wasm.has_own_foo_property({ foo: 42 }));
                assert.ok(!wasm.has_own_foo_property({}));
            }
        "#)
        .test()
    }
}
