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

            export function test() {
                assert.ok(wasm.symbol_has_instance());
                assert.ok([] instanceof Array);
            }
        "#,
        )
        .test()
}
