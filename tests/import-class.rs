extern crate test_support;

#[test]
fn simple() {
    test_support::project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            wasm_bindgen! {
                extern struct Math {
                    fn random() -> f64;
                }

                pub fn get_random() -> f64 {
                    Math::random()
                }
            }
        "#)
        .file("test.ts", r#"
            import * as wasm from "./out";

            export function test() {
                wasm.get_random();
            }
        "#)
        .test();
}
