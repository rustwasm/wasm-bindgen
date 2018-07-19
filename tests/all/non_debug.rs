use super::project;

#[test]
fn works() {
    project()
        .debug(false)
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros, wasm_import_module)]

                extern crate wasm_bindgen;

                use wasm_bindgen::prelude::*;

                #[wasm_bindgen]
                pub struct A {}

                #[wasm_bindgen]
                impl A {
                    pub fn new() -> A {
                        A {}
                    }
                }

                #[wasm_bindgen]
                pub fn clone(a: &JsValue) -> JsValue {
                    drop(a.clone());
                    a.clone()
                }
            "#,
        )
        .file(
            "test.js",
            r#"
                import * as assert from "assert";
                import * as wasm from "./out";

                export function test() {
                    let sym = Symbol('a');
                    assert.strictEqual(wasm.clone(sym), sym);
                    let a = wasm.A.new();
                    a.free();
                }
            "#,
        )
        .test();
}
