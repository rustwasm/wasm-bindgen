use super::project;

#[test]
fn same_function_different_locations() {
    project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros)]

                extern crate wasm_bindgen;
                use wasm_bindgen::prelude::*;

                pub mod a {
                    use wasm_bindgen::prelude::*;
                    #[wasm_bindgen(module = "./foo")]
                    extern {
                        pub fn foo();
                    }
                }

                pub mod b {
                    use wasm_bindgen::prelude::*;
                    #[wasm_bindgen(module = "./foo")]
                    extern {
                        pub fn foo();
                    }
                }

                #[wasm_bindgen]
                pub fn test() {
                    a::foo();
                    b::foo();
                }
            "#,
        )
        .file("foo.js", "export function foo() {}")
        .test();
}

#[test]
fn same_function_different_modules() {
    project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros)]

                extern crate wasm_bindgen;
                use wasm_bindgen::prelude::*;

                pub mod a {
                    use wasm_bindgen::prelude::*;
                    #[wasm_bindgen(module = "./foo")]
                    extern {
                        pub fn foo() -> bool;
                    }
                }

                pub mod b {
                    use wasm_bindgen::prelude::*;
                    #[wasm_bindgen(module = "./bar")]
                    extern {
                        pub fn foo() -> bool;
                    }
                }

                #[wasm_bindgen]
                pub fn test() {
                    assert!(a::foo());
                    assert!(!b::foo());
                }
            "#,
        )
        .file("foo.js", "export function foo() { return true; } ")
        .file("bar.js", "export function foo() { return false; } ")
        .test();
}
