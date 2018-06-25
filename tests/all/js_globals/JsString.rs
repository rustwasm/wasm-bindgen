#![allow(non_snake_case)]

use project;

#[test]
fn char_at() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn string_char_at(this: &js::JsString, index: u32) -> js::JsString {
                this.char_at(index)
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            var anyString = 'Brave new world';

            export function test() {
                assert.equal(wasm.string_char_at(anyString, 0), "B");
                assert.equal(wasm.string_char_at(anyString, 999), "");
            }
        "#)
        .test()
}

#[test]
fn slice() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn create_slice(this: &js::JsString, start: u32, end: u32) -> js::JsString {
                this.slice(start, end)
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let characters = "acxn18";
                let subset = wasm.create_slice(characters, 1, 3);

                assert.equal(subset, "cx");
            }
        "#)
        .test()
}

#[test]
fn substr() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn create_substr(this: &js::JsString, start: i32, length: i32) -> js::JsString {
                this.substr(start, length)
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let aString = "Mozilla";

                assert.equal(wasm.create_substr(aString, 0, 1), "M");
                assert.equal(wasm.create_substr(aString, 1, 0), "");
                assert.equal(wasm.create_substr(aString, -1, 1), "a");
                assert.equal(wasm.create_substr(aString, 1, -1), "");
                // TODO: Uncomment and test these assertions, once we have support for optional parameters
                // assert.equal(wasm.create_substr(aString, -3), "lla");
                // assert.equal(wasm.create_substr(aString, 1), "ozilla");
                assert.equal(wasm.create_substr(aString, -20, 2), "Mo");
                assert.equal(wasm.create_substr(aString, 20, 2), "");
            }
        "#)
        .test()
}