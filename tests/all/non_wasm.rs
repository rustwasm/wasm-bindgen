use super::{project, run};
use std::process::Command;

#[test]
fn works() {
    let mut p = project();
    let name = p.crate_name();
    p.rlib(true)
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros, wasm_import_module)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            pub struct A {
                x: u32,
            }

            #[wasm_bindgen]
            impl A {
                pub fn new() -> A {
                    A { x: 3 }
                }

                pub fn foo(&self) {
                }
            }

            #[wasm_bindgen]
            pub fn foo(x: bool) {
                A::new().foo();

                if x {
                    bar("test");
                    baz(JsValue::from(3));
                }
            }

            #[wasm_bindgen]
            extern {
                fn some_import();
                static A: JsValue;
            }

            #[wasm_bindgen]
            pub fn bar(_: &str) -> JsValue {
                some_import();
                A.clone()
            }

            #[wasm_bindgen]
            pub fn baz(_: JsValue) {
            }
        "#,
        )
        .file(
            "tests/foo.rs",
            &format!(
                "
            extern crate {} as mytest;

            #[test]
            fn foo() {{
                mytest::foo(false);
                mytest::A::new().foo();
            }}
        ",
                name
            ),
        )
        .file(
            "benches/foo.rs",
            &format!(
                "
            #![feature(test)]
            extern crate test;
            extern crate {} as mytest;

            #[bench]
            fn foo(b: &mut test::Bencher) {{
                b.iter(|| mytest::foo(false));
            }}
        ",
                name
            ),
        );
    let (root, target_dir) = p.build();
    let mut cmd = Command::new("cargo");
    cmd.arg("test")
        .arg("--test")
        .arg("foo")
        .arg("--bench")
        .arg("foo")
        .current_dir(&root)
        .env("CARGO_TARGET_DIR", &target_dir);
    run(&mut cmd, "cargo");
}
