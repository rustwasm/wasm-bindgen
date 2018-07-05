use super::project;

#[test]
fn dependencies_work() {
    project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(proc_macro, wasm_custom_section, wasm_import_module)]
                extern crate wasm_bindgen;
                extern crate dependency;
                use wasm_bindgen::prelude::*;

                #[wasm_bindgen]
                pub fn return_dep_ty(x: f64) -> dependency::Foo {
                    dependency::Foo(x)
                }

                #[wasm_bindgen]
                pub fn takes_own_dep_ty(foo: dependency::Foo) -> f64 {
                    foo.0
                }

                #[wasm_bindgen]
                pub fn takes_ref_dep_ty(foo: &dependency::Foo) -> f64 {
                    foo.0
                }

                #[wasm_bindgen]
                pub fn takes_mut_dep_ty(foo: &mut dependency::Foo, x: f64) {
                    foo.0 = x;
                }
            "#,
        )
        .add_local_dependency("dependency", "vendor/dependency")
        .file(
            "vendor/dependency/Cargo.toml",
            &format!(
                r#"
                    [package]
                    name = "dependency"
                    version = "0.0.1"
                    authors = []

                    [dependencies]
                    wasm-bindgen = {{ path = '{}' }}
            "#,
                env!("CARGO_MANIFEST_DIR")
            ),
        )
        .file(
            "vendor/dependency/src/lib.rs",
            r#"
                #![feature(proc_macro, wasm_custom_section, wasm_import_module)]
                extern crate wasm_bindgen;
                use wasm_bindgen::prelude::*;

                #[wasm_bindgen]
                pub struct Foo(pub f64);

                #[wasm_bindgen]
                impl Foo {
                    pub fn new(x: f64) -> Foo { Foo(x) }
                    pub fn get(&self) -> f64 { self.0 }
                }
            "#,
        )
        .file(
            "test.js",
            r#"
                import * as assert from "assert";
                import * as wasm from "./out";

                export function test() {
                    const foo = wasm.return_dep_ty(42);
                    assert.strictEqual(foo.get(), 42);

                    const x = wasm.takes_ref_dep_ty(foo);
                    assert.strictEqual(x, 42);

                    const y = 1337;
                    wasm.takes_mut_dep_ty(foo, y);
                    assert.strictEqual(foo.get(), y);

                    const z = wasm.takes_own_dep_ty(foo);
                    assert.strictEqual(z, y);
                    assert.strictEqual(foo.ptr, 0);
                }
            "#,
        )
        .test();
}
