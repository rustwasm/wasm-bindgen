#![allow(non_snake_case)]

use project;

#[test]
fn apply() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn apply(target: &js::Function, this_argument: &JsValue, arguments_list: &js::Array) -> JsValue {
                let result = js::Reflect::apply(target, this_argument, arguments_list);
                let result = match result {
                    Ok(val) => val,
                    Err(_err) => "TypeError".into()
                };
                result
            }
        "#,
        )
        .file(
            "test.ts",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.apply(''.charAt, 'ponies', [3]), 'i');
                assert.equal(wasm.apply('', 'ponies', [3]), "TypeError");
            }
        "#,
        )
        .test()
}