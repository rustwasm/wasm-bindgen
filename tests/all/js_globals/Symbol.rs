#![allow(non_snake_case)]

use project;

#[test]
fn has_instance() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn symbol_has_instance() -> js::Symbol {
                js::Symbol::has_instance()
            }

        "#,
        )
        .file(
            "test.ts",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";
            class Array1 {}
            Object.defineProperty(Array1, wasm.symbol_has_instance(), {
                value: (instance: any) => Array.isArray(instance)
            });

            export function test() {
                assert.equal(typeof wasm.symbol_has_instance(), "symbol");
                assert.ok([] instanceof Array1);
            }
        "#,
        )
        .test()
}
