#![allow(non_snake_case)]

use super::project;

#[test]
fn value_of() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::Date;

            #[wasm_bindgen]
            pub fn js_value_of(this: &Date) -> Date {
                this.value_of()
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let date = new Date(Date.UTC(96, 1, 2, 3, 4, 5));
                assert.equal(wasm.js_value_of(date), 823230245000);
            }
        "#)
        .test()
}
