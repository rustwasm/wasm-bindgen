#![allow(non_snake_case)]

use project;

#[test]
fn keys() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn get_keys(this: &js::Array) -> js::ArrayIterator {
                this.keys()
            }

        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let characters = [8, 5, 4, 3, 1, 2]
                let iterator = characters.keys();
                let wasmIterator = wasm.get_keys(characters);

                assert.equal(iterator.toString(), wasmIterator.toString());
                assert.equal(Array.from(iterator)[0], Array.from(wasmIterator)[0]);
            }
        "#,
        )
        .test()
}

#[test]
fn entries() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn get_entries(this: &js::Array) -> js::ArrayIterator {
                this.entries()
            }

        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let characters = [8, 5, 4, 3, 1, 2]
                let iterator = characters.entries();
                let wasmIterator = wasm.get_entries(characters);
                let jsItem = iterator.next();
                let wasmItem = wasmIterator.next();

                assert.equal(iterator.toString(), wasmIterator.toString());
                assert.equal(jsItem.value[1], wasmItem.value[1]);
            }
        "#,
        )
        .test()
}
