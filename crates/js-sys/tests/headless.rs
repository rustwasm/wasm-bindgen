#![cfg(not(target_arch = "wasm32"))]
#![allow(non_snake_case)]

extern crate wasm_bindgen_test_project_builder as project_builder;

fn project() -> project_builder::Project {
    let mut p = project_builder::project();
    p.add_local_dependency("js-sys", env!("CARGO_MANIFEST_DIR"));
    return p
}

// NB: currently this older test suite is only used for tests which require
// headless browser support, otherwise all new tests should go in the `wasm`
// test suite next to this one.

#[test]
fn ArrayIterator_values() {
    let mut project = project();
    project.file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            extern crate js_sys;
            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            pub fn get_values(this: &js_sys::Array) -> js_sys::Iterator {
                this.values()
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                if (typeof Array.prototype.values !== "function") {
                    return;
                }

                let numbers = [8, 3, 2];
                let wasmIterator = wasm.get_values(numbers);

                assert.equal(wasmIterator.next().value, 8);
                assert.equal(wasmIterator.next().value, 3);
                assert.equal(wasmIterator.next().value, 2);
                assert.ok(wasmIterator.next().done);
            }
        "#,
        );

    let mut headless = project.clone();
    headless.headless(true);

    project.test();
    headless.test();
}
