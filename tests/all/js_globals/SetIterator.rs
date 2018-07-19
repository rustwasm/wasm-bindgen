#![allow(non_snake_case)]

use project;

#[test]
fn entries() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn entries(this: &js::Set) -> js::SetIterator {
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
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn keys(this: &js::Set) -> js::SetIterator {
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
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn values(this: &js::Set) -> js::SetIterator {
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
