extern crate test_support;

#[test]
fn works() {
    test_support::project()
        .debug(false)
        .file("src/lib.rs", r#"
            #![feature(proc_macro)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            wasm_bindgen! {
                pub struct A {}

                impl A {
                    pub fn new() -> A {
                        A {}
                    }
                }
                pub fn clone(a: &JsObject) -> JsObject {
                    drop(a.clone());
                    a.clone()
                }
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let sym = (Symbol as any)('a');
                assert.strictEqual(wasm.clone(sym), sym);
                let a = wasm.A.new();
                a.free();
            }
        "#)
        .test();
}
