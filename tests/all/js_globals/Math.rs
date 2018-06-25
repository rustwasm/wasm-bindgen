#![allow(non_snake_case)]

use super::project;


#[test]
fn abs() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn abs(number: i32) -> js::Number {
                js::Math::abs(number)
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.abs(-32), Math.abs(-32));
                assert.equal(wasm.abs(32), 32));
            }
        "#)
        .test()
}
