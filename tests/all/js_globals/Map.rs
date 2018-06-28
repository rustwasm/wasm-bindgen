#![allow(non_snake_case)]

use project;

#[test]
fn clear() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn map_clear(this: &js::Map) {
                this.clear();
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                const map = new Map();
                map.set('foo', 'bar');
                map.set('bar', 'baz');
                assert.equal(map.size, 2);
                wasm.map_clear(map);
                assert.equal(map.size, 0);

            }
        "#)
        .test()
}