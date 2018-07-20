#![allow(non_snake_case)]

use super::project;

#[test]
fn values() {
    let mut project = project();
    project.file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            extern crate js_sys;
            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            pub fn get_values(this: &js_sys::Array) -> js_sys::ArrayIterator {
                this.values()
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                if (typeof Array.prototype.values !== "function") {
                    return;
                }

                let numbers = [8, 3, 2];
                let wasmIterator = wasm.get_values(numbers);

                assert.equal(wasmIterator.next().value, 8);
                assert.equal(wasmIterator.next().value, 3);
                assert.equal(wasmIterator.next().value, 2);
                assert.ok(wasmIterator.next().done);
            }
        "#,
        );

    let mut headless = project.clone();
    headless.headless(true);

    project.test();
    headless.test();
}
