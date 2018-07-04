#![allow(non_snake_case)]

use super::project;

#[test]
fn validate() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use JsValue;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::WebAssembly;

            #[wasm_bindgen]
            pub fn validate_wasm(wasm: JsValue) -> bool {
                WebAssembly::validate(wasm).unwrap_or(false)
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.validate_wasm(new ArrayBuffer(42)), false);
            }
        "#)
        .test()
}
