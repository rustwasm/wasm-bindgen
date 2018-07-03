#![allow(non_snake_case)]

use project;

#[test]
fn next() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn next(this: &js::Generator, value: &JsValue) -> JsValue {
                this.next(value)
            }
        "#,
        )
        .file(
            "test.ts",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                function* generator() {
                    const reply = yield '2 * 2';

                    return reply === 4;
                }

                const gen = generator();

                const q = wasm.next(gen, undefined);

                assert.equal(q.value, '2 * 2');
                assert.equal(q.done, false);

                const a = wasm.next(gen, 4);

                assert.equal(a.value, true);
                assert.equal(a.done, true);
            }
        "#,
        )
        .test()
}
