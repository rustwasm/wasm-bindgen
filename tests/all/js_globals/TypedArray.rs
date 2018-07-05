#![allow(non_snake_case)]

use project;

#[test]
fn new_undefined() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn new_array() -> js::Uint8Array {
                js::Uint8Array::new(JsValue::undefined())
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.new_array().length, 0);
            }
        "#,
        )
        .test()
}

#[test]
fn new_length() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn new_array() -> js::Uint8Array {
                js::Uint8Array::new(JsValue::from_f64(4.0))
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.new_array().length, 4);
            }
        "#,
        )
        .test()
}

#[test]
fn fill() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn fill_with(this: &js::Uint8Array, value: JsValue, start: u32, end: u32) -> js::Uint8Array {
                this.fill(value, start, end)
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let characters = new Uint8Array([0, 0, 0, 0, 0, 0]);
                let subset = wasm.fill_with(characters, 1, 0, 3);

                assert.equal(subset[0], 1);
                assert.equal(subset[4], 0);
            }
        "#)
        .test()
}
