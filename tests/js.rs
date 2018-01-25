extern crate test_support;

const SRC: &str = r#"
    #![feature(proc_macro)]

    extern crate wasm_bindgen;

    use wasm_bindgen::prelude::*;

    wasm_bindgen! {
        pub struct A {}

        impl A {
            pub fn new() -> A {
                A {}
            }
        }
        pub fn clone(a: &JsObject) -> JsObject {
            drop(a.clone());
            a.clone()
        }

        extern "JS" {
            fn bar(a: &JsObject, b: JsObject);
        }

        pub fn foo(
            _: &str,
            _: bool,
            _: i32,
            _: &A,
            _: A,
            a: JsObject,
            b: &JsObject,
        ) -> String {
            a.is_symbol();
            a.as_f64();
            a.as_string();
            a.as_bool();
            a.is_null();
            a.is_undefined();
            bar(b, a);
            JsObject::from("a");
            JsObject::from(3);
            String::new()
        }
    }
"#;

#[test]
fn works() {
    test_support::project()
        .js(true)
        .debug(true)
        .file("src/lib.rs", SRC)
        .file("test.ts", r#"
            export const imports = {};

            export function test(_) {
            }
        "#)
        .test();
}

#[test]
fn works_non_debug() {
    test_support::project()
        .js(true)
        .debug(false)
        .file("src/lib.rs", SRC)
        .file("test.ts", r#"
            export const imports = {};

            export function test(_) {
            }
        "#)
        .test();
}

