use super::project;

#[test]
fn add_headless() {
    project()
        .add_local_dependency("web-sys", env!("CARGO_MANIFEST_DIR"))
        .headless(true)
        .file(
            "src/lib.rs",
            r#"
                #![feature(proc_macro, wasm_custom_section)]
                extern crate wasm_bindgen;
                use wasm_bindgen::prelude::*;

                #[wasm_bindgen]
                pub fn add(a: u32, b: u32) -> u32 {
                    a + b
                }
            "#,
        )
        .file(
            "test.js",
            r#"
                import * as assert from "assert";
                import * as wasm from "./out";

                export function test() {
                    console.log("start `add_headless` test");
                    assert.strictEqual(wasm.add(1, 2), 3);
                    console.log("end `add_headless` test");
               }
            "#,
        )
        .test();
}
