#![allow(non_snake_case)]

use super::project;

#[test]
fn validate() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use JsValue;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::WebAssembly;

            #[wasm_bindgen]
            pub fn validate_wasm(wasm: JsValue) -> JsValue {
                match WebAssembly::validate(wasm) {
                    Ok(value) => value.into(),
                    Err(err) => err
                }
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.validate_wasm(new ArrayBuffer(42)), false);
            }
        "#)
        .test()
}

#[test]
fn validate_with_invalid_input() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use JsValue;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::WebAssembly;

            #[wasm_bindgen]
            pub fn validate_wasm(wasm: JsValue) -> JsValue {
                match WebAssembly::validate(wasm) {
                    Ok(value) => value.into(),
                    Err(err) => err
                }
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                try {
                    wasm.validate_wasm(42);
                    assert.ok(false);
                } catch (e) {
                    assert.ok(true);
                }
            }
        "#)
        .test()
}
