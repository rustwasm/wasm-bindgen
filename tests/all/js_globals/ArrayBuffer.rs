#![allow(non_snake_case)]

use super::project;

#[test]
fn new() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::ArrayBuffer;

            #[wasm_bindgen]
            pub fn new_arraybuffer() -> ArrayBuffer {
                ArrayBuffer::new(42f64)
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(typeof wasm.new_arraybuffer(), "object");
            }
        "#)
        .test()
}

#[test]
fn is_view() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use JsValue;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::ArrayBuffer;

            #[wasm_bindgen]
            pub fn is_uint8array(value: JsValue) -> bool {
                ArrayBuffer::is_view(value)
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.is_uint8array(new Uint8Array(42)), true);
            }
        "#)
        .test()
}

#[test]
fn slice() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::ArrayBuffer;

            #[wasm_bindgen]
            pub fn slice_at_2(arraybuffer: ArrayBuffer) -> ArrayBuffer {
                ArrayBuffer::slice(&arraybuffer, 2f64)
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                const arraybuffer = new ArrayBuffer(4);
                assert.equal(typeof wasm.slice_at_2(arraybuffer), "object");
            }
        "#)
        .test()
}
