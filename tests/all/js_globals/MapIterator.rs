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
            pub fn get_entries(this: &js::Map) -> js::MapIterator {
                this.entries()
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                const map = new Map();
                const iterator = map.entries();
                const wasmIterator = wasm.get_entries(map);
                map.set('foo', 'bar');
                map.set('bar', 'baz');

                assert.equal(iterator.toString(), wasmIterator.toString());
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
            pub fn get_keys(this: &js::Map) -> js::MapIterator {
                this.keys()
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                const map = new Map();
                const iterator = map.keys();
                const wasmIterator = wasm.get_keys(map);
                map.set('foo', 'bar');
                map.set('bar', 'baz');

                assert.equal(iterator.toString(), wasmIterator.toString());
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
            pub fn get_values(this: &js::Map) -> js::MapIterator {
                this.values()
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                const map = new Map();
                const iterator = map.keys();
                const wasmIterator = wasm.get_values(map);
                map.set('foo', 'bar');
                map.set('bar', 'baz');

                assert.equal(iterator.toString(), wasmIterator.toString());
            }
        "#)
        .test()
}
