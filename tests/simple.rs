extern crate test_support;

#[test]
fn add() {
    test_support::project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            wasm_bindgen! {
                pub fn add(a: u32, b: u32) -> u32 {
                    a + b
                }
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";

            export function test(wasm) {
                assert.strictEqual(wasm.instance.exports.add(1, 2), 3);
                assert.strictEqual(wasm.instance.exports.add(2, 3), 5);
            }
        "#)
        .test();
}
