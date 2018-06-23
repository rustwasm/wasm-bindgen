#![allow(non_snake_case)]

use project;

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
