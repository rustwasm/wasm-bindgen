use super::project;

#[test]
fn same_api_two_crates() {
    project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros)]
                extern crate wasm_bindgen;
                extern crate a;
                extern crate b;

                use wasm_bindgen::prelude::*;

                #[wasm_bindgen(module = "./foo")]
                extern {
                    fn assert_next_undefined();
                    fn assert_next_ten();
                }

                #[wasm_bindgen]
                pub fn test() {
                    assert_next_undefined();
                    a::test();
                    assert_next_ten();
                    b::test();
                }
            "#,
        )
        .file(
            "foo.js",
            r#"
                import { strictEqual } from "assert";

                let next = null;

                export function assert_next_undefined() {
                    next = undefined;
                }

                export function assert_next_ten() {
                    next = 10;
                }

                export function foo(a) {
                    console.log(a, next);
                    strictEqual(a, next);
                    next = null;
                }
            "#,
        )
        .add_local_dependency("a", "a")
        .file(
            "a/Cargo.toml",
            &format!(r#"
                [package]
                name = 'a'
                version = '0.0.0'

                [dependencies]
                wasm-bindgen = {{ path = '{}' }}
            "#,
                env!("CARGO_MANIFEST_DIR")
            ),
        )
        .file(
            "a/src/lib.rs",
            "
                #![feature(use_extern_macros)]
                extern crate wasm_bindgen;

                use wasm_bindgen::prelude::*;

                #[wasm_bindgen(module = \"./foo\")]
                extern {
                    fn foo();
                }

                pub fn test() {
                    foo();
                }
            ",
        )
        .add_local_dependency("b", "b")
        .file(
            "b/Cargo.toml",
            &format!(r#"
                [package]
                name = 'b'
                version = '0.0.0'

                [dependencies]
                wasm-bindgen = {{ path = '{}' }}
            "#,
                env!("CARGO_MANIFEST_DIR")
            ),
        )
        .file(
            "b/src/lib.rs",
            "
                #![feature(use_extern_macros)]
                extern crate wasm_bindgen;

                use wasm_bindgen::prelude::*;

                #[wasm_bindgen(module = \"./foo\")]
                extern {
                    fn foo(x: u32);
                }

                pub fn test() {
                    foo(10);
                }
            ",
        )
        .test();
}
