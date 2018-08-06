use super::project;

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

