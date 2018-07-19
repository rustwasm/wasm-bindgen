use super::project;

#[test]
fn works() {
    project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros, wasm_import_module)]
                extern crate wasm_bindgen;

                use wasm_bindgen::prelude::*;

                #[wasm_bindgen]
                extern {
                    pub type Foo;

                    #[wasm_bindgen(method, structural)]
                    fn bar(this: &Foo);
                    #[wasm_bindgen(method, getter, structural)]
                    fn baz(this: &Foo) -> u32;
                    #[wasm_bindgen(method, setter, structural)]
                    fn set_baz(this: &Foo, val: u32);
                }

                #[wasm_bindgen]
                pub fn run(a: &Foo) {
                    a.bar();
                    assert_eq!(a.baz(), 1);
                    a.set_baz(2);
                    assert_eq!(a.baz(), 2);
                }
            "#,
        )
        .file(
            "test.js",
            r#"
                import * as assert from "assert";
                import { run } from "./out";

                export function test() {
                    let called = false;
                    run({
                        bar() { called = true; },
                        baz: 1,
                    });
                    assert.strictEqual(called, true);
                }
            "#,
        )
        .test();
}
