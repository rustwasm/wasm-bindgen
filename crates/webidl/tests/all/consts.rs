use super::project;

#[test]
fn bool() {
    project()
        .file(
            "foo.webidl",
            r#"
                interface Foo {
                    const boolean not_true = false;
                    const boolean not_false = true;
                };
            "#,
        )
        // a corresponding const in the js implementation is not required
        // value is taken directly from idl
        .file(
            "foo.js",
            r#"
                export class Foo {
                }
            "#,
        )
        .file(
            "src/lib.rs",
            r#"
                #![feature(proc_macro, wasm_custom_section, wasm_import_module)]
                extern crate wasm_bindgen;
                use wasm_bindgen::prelude::*;

                pub mod foo;
                use foo::Foo;

                #[wasm_bindgen]
                pub fn test() {
                    let falsish: bool = Foo::NOT_TRUE;
                    assert!(!falsish);
                    let trueish: bool = Foo::NOT_FALSE;
                    assert!(trueish);
                }
            "#,
        )
        .test();
}
