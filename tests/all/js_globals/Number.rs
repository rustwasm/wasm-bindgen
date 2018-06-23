#![allow(non_snake_case)]

use super::project;


#[test]
fn value_of() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn js_value_of(this: &js::Number) -> js::Number {
                this.value_of()
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let number = 42;
                assert.equal(wasm.js_value_of(number), 42);
                assert.equal(typeof wasm.js_value_of(number), "number");
            }
        "#)
        .test()
}
