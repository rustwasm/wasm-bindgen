use super::project;

#[test]
fn works() {
    project()
        .webpack(true)
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros, wasm_import_module)]

                extern crate wasm_bindgen;

                use wasm_bindgen::prelude::*;

                #[wasm_bindgen]
                pub fn foo() {}

                #[wasm_bindgen]
                pub fn bar(a: &str, b: u32) -> String {
                    format!("{} {}", a, b)
                }

                #[wasm_bindgen]
                pub fn thunk(a: &JsValue) {
                    drop(a);
                }

                #[wasm_bindgen]
                pub struct A {
                }

                #[wasm_bindgen]
                impl A {
                    #[wasm_bindgen(constructor)]
                    pub fn new() -> A {
                        A {}
                    }

                    pub fn new2() -> A {
                        A {}
                    }

                    pub fn foo(&self) {}

                    pub fn bar(&self, _a: u32) {}

                    pub fn baz(&self, _d: &A) {}
                }
            "#,
        )
        .file(
            "test.ts",
            r#"
                import * as assert from 'assert';
                import { foo, bar, A, thunk } from './out';
                import { memory } from './out_bg';

                export function test() {
                    foo();
                    assert.strictEqual(bar('a', 3), 'a 3');

                    const x = new A();
                    x.foo();
                    x.free();

                    const y = A.new2();
                    y.foo();
                    y.bar(2);
                    y.baz(y);
                    y.free();

                    thunk(memory);
                };
            "#,
        )
        .test();
}

