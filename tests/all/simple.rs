use super::project;

#[test]
fn add_headless() {
    project()
        .headless(true)
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros)]
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

#[test]
fn no_std() {
    project()
        .no_std(true)
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros)]
                #![no_std]
                #![allow(dead_code)]

                extern crate wasm_bindgen;
                extern crate std as _some_other_name;

                use wasm_bindgen::prelude::*;

                #[wasm_bindgen(module = "./foo")]
                extern {
                    fn test(a: &str);

                    type Js;
                    #[wasm_bindgen(constructor)]
                    fn new() -> Js;
                    #[wasm_bindgen(method)]
                    fn init(this: &Js);
                }

                #[wasm_bindgen]
                pub fn foo(_a: u32) {}
            "#,
        )
        .file(
            "test.js",
            r#"
                import * as wasm from "./out_bg";

                export function test() {
                    // mostly just testing the project compiles here
                    wasm.foo(1);
                }
            "#,
        )
        .file(
            "foo.js",
            r#"
                export class Js {
                    init() {
                    }
                }
            "#,
        )
        .test();
}

#[test]
fn no_std_class() {
    project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros)]
                #![no_std]
                #![allow(dead_code)]

                extern crate wasm_bindgen;
                extern crate std as _some_other_name;

                use wasm_bindgen::prelude::*;

                #[wasm_bindgen]
                extern {
                    fn test(a: &str);

                    type Js;
                    #[wasm_bindgen(constructor)]
                    fn new() -> Js;
                    #[wasm_bindgen(method, structural)]
                    fn init(this: &Js);
                }

                #[wasm_bindgen]
                pub fn foo(_a: u32) {}

                #[wasm_bindgen]
                pub struct A {}

                #[wasm_bindgen]
                impl A {
                    pub fn foo(&self) {}
                    pub fn bar(&mut self) {}
                }
            "#,
        )
        .file(
            "test.js",
            r#"
                import * as wasm from "./out_bg";

                export function test() {
                    // mostly just testing the project compiles here
                    wasm.foo(1);
                }
            "#,
        )
        .test();
}

