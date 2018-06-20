#![allow(non_snake_case)]

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

#[test]
fn to_string() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn to_string(obj: &js::Object) -> String {
                obj.to_string()
            }

            #[wasm_bindgen]
            pub fn test() {
                let object = js::Object::new();
                assert_eq!(object.to_string(), "[object Object]");
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.to_string({ foo: 42 }), "[object Object]");
                wasm.test();
            }
        "#)
        .test()
}
