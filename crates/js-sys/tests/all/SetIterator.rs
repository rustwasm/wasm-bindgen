#![allow(non_snake_case)]

use project;

#[test]
fn entries() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            extern crate js_sys;
            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            pub fn entries(this: &js_sys::Set) -> js_sys::SetIterator {
                this.entries()
            }

        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let set = new Set([8, 5, 4, 3, 1, 2]);
                let wasmIterator = wasm.entries(set);
                let nextValue = wasmIterator.next().value;

                assert.equal(nextValue[0], 8);
                assert.equal(nextValue[1], 8);
            }
        "#)
        .test()
}

#[test]
fn keys() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            extern crate js_sys;
            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            pub fn keys(this: &js_sys::Set) -> js_sys::SetIterator {
                this.keys()
            }

        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let set = new Set([8, 5, 4, 3, 1, 2]);
                let wasmIterator = wasm.keys(set);
                let nextValue = wasmIterator.next().value;

                assert.equal(nextValue, 8);
            }
        "#)
        .test()
}

#[test]
fn values() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            extern crate js_sys;
            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            pub fn values(this: &js_sys::Set) -> js_sys::SetIterator {
                this.values()
            }

        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let set = new Set([8, 5, 4, 3, 1, 2]);
                let wasmIterator = wasm.values(set);
                let nextValue = wasmIterator.next().value;

                assert.equal(nextValue, 8);
            }
        "#)
        .test()
}
