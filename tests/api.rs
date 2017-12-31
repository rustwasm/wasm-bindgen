extern crate test_support;

#[test]
fn works() {
    test_support::project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            wasm_bindgen! {
                pub fn foo() -> JsObject {
                    JsObject::from("foo")
                }

                pub fn bar(s: &str) -> JsObject {
                    JsObject::from(s)
                }
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import { Exports, Imports } from "./out";

            export const imports: Imports = {};

            export function test(wasm: Exports) {
                assert.strictEqual(wasm.foo(), 'foo');
                assert.strictEqual(wasm.bar('a'), 'a');
            }
        "#)
        .test();
}

