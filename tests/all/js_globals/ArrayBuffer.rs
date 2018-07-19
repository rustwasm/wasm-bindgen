#![allow(non_snake_case)]

use super::project;

#[test]
fn new() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::ArrayBuffer;

            #[wasm_bindgen]
            pub fn new_arraybuffer() -> ArrayBuffer {
                ArrayBuffer::new(42)
            }
        "#)
        .file("test.js", r#"
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
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use JsValue;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::ArrayBuffer;

            #[wasm_bindgen]
            pub fn is_view(value: JsValue) -> bool {
                ArrayBuffer::is_view(value)
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.is_view(new Uint8Array(42)), true);
            }
        "#)
        .test()
}

#[test]
fn slice() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::ArrayBuffer;

            #[wasm_bindgen]
            pub fn slice(arraybuffer: &ArrayBuffer, begin: u32) -> ArrayBuffer {
                arraybuffer.slice(begin)
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                const arraybuffer = new ArrayBuffer(4);
                assert.equal(typeof wasm.slice(arraybuffer, 2), "object");
            }
        "#)
        .test()
}

#[test]
fn slice_with_end() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::ArrayBuffer;

            #[wasm_bindgen]
            pub fn slice_with_end(arraybuffer: &ArrayBuffer, begin: u32, end: u32) -> ArrayBuffer {
                arraybuffer.slice_with_end(begin, end)
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                const arraybuffer = new ArrayBuffer(4);
                assert.equal(typeof wasm.slice_with_end(arraybuffer, 1, 2), "object");
            }
        "#)
        .test()
}
