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
            pub fn string_char_at(string: &js::String, index: i32) -> js::String {
                string.char_at(index)
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