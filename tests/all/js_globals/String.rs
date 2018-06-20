#![allow(non_snake_case)]

use project;

#[test]
fn length() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn get_length(val: &js::String) -> i32 {
                val.length()
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let string = 'hello';
                assert.equal(wasm.get_length(string), 5);
            }
        "#)
        .test()
}
