extern crate test_support;

#[test]
fn works() {
    test_support::project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section, wasm_import_module)]

            extern crate wasm_bindgen;

            use std::cell::Cell;
            use wasm_bindgen::prelude::*;

            #[wasm_bindgen(module = "./test")]
            extern {
                fn call(a: &Fn());
                fn thread(a: &Fn(u32) -> u32) -> u32;
            }

            #[wasm_bindgen]
            pub fn run() {
                let a = Cell::new(false);
                call(&|| a.set(true));
                assert!(a.get());

                assert_eq!(thread(&|a| a + 1), 3);
            }
        "#)
        .file("test.ts", r#"
            import { run } from "./out";

            export function call(a: any) {
                a();
            }

            export function thread(a: any) {
                return a(2);
            }

            export function test() {
                run();
            }
        "#)
        .test();
}

#[test]
fn cannot_reuse() {
    test_support::project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section, wasm_import_module)]

            extern crate wasm_bindgen;

            use std::cell::Cell;
            use wasm_bindgen::prelude::*;

            #[wasm_bindgen(module = "./test")]
            extern {
                fn call(a: &Fn());
                #[wasm_bindgen(catch)]
                fn call_again() -> Result<(), JsValue>;
            }

            #[wasm_bindgen]
            pub fn run() {
                call(&|| {});
                assert!(call_again().is_err());
            }
        "#)
        .file("test.ts", r#"
            import { run } from "./out";

            let CACHE: any = null;

            export function call(a: any) {
                CACHE = a;
            }

            export function call_again() {
                CACHE();
            }

            export function test() {
                run();
            }
        "#)
        .test();
}

