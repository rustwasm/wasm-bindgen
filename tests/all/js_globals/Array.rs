#![allow(non_snake_case)]

use project;

#[test]
fn index_of() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn get_index_of(this: &js::Array, value: JsValue, from_index: i32) -> i32 {
                this.index_of(value, from_index)
            }

        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let characters = ["a", "c", "x", "n"];
                let index = wasm.get_index_of(characters, "x", 0);
                let notFoundIndex = wasm.get_index_of(characters, "z", 0);

                assert.equal(index, 2);
                assert.equal(notFoundIndex, -1);

                let withFromIndex = wasm.get_index_of(characters, "x", -3);
                let withFromIndexNotFound = wasm.get_index_of(characters, "a", -2);

                assert.equal(withFromIndex, 2);
                assert.equal(withFromIndexNotFound, -1);
            }
        "#)
        .test()
}

#[test]
fn last_index_of() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn get_last_index_of(this: &js::Array, value: JsValue, from_index: i32) -> i32 {
                this.last_index_of(value, from_index)
            }

        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let characters = ["a", "x", "c", "x", "n"];
                let index = wasm.get_last_index_of(characters, "x", 5);
                let notFoundIndex = wasm.get_last_index_of(characters, "z", 5);

                assert.equal(index, 3);
                assert.equal(notFoundIndex, -1);

                let withFromIndex = wasm.get_last_index_of(characters, "x", 2);
                let withFromIndexNotFound = wasm.get_last_index_of(characters, "x", 0);

                assert.equal(withFromIndex, 1);
                assert.equal(withFromIndexNotFound, -1);
            }
        "#)
        .test()
}