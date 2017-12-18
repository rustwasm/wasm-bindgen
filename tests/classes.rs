extern crate test_support;

#[test]
fn simple() {
    test_support::project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            wasm_bindgen! {
                pub struct Foo {
                    contents: u32,
                }

                impl Foo {
                    pub fn new() -> Foo {
                        Foo::with_contents(0)
                    }

                    pub fn with_contents(a: u32) -> Foo {
                        Foo::with_contents(a)
                    }

                    pub fn add(&mut self, amt: u32) -> u32 {
                        self.contents += amt;
                        self.contents
                    }
                }
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";

            export function test(wasm) {
                const r = new wasm.Foo();
                assert.strictEqual(r.add(0), 0);
                assert.strictEqual(r.add(1), 1);
                assert.strictEqual(r.add(1), 2);

                const r2 = wasm.Foo.with_contents(10);
                assert.strictEqual(r.add(1), 11);
                assert.strictEqual(r.add(2), 13);
                assert.strictEqual(r.add(3), 16);
            }
        "#)
        .test();
}
