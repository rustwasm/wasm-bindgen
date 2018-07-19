#![allow(non_snake_case)]

use project;

#[test]
fn new_undefined() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            extern crate js_sys;
            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            pub fn new_boolean() -> js_sys::Boolean {
                js_sys::Boolean::new(JsValue::undefined())
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.new_boolean().valueOf(), false);
            }
        "#,
        )
        .test()
}

#[test]
fn new_truely() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            extern crate js_sys;
            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            pub fn new_boolean() -> js_sys::Boolean {
                js_sys::Boolean::new(JsValue::from("foo"))
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.new_boolean().valueOf(), true);
            }
        "#,
        )
        .test()
}
