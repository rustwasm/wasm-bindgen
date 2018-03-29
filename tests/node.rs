extern crate test_support;

#[test]
fn works() {
    test_support::project()
        .node(true)
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section, wasm_import_module)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            #[wasm_bindgen(module = "./test")]
            extern {
                fn hit();
            }

            #[wasm_bindgen]
            pub fn run() {
                hit();
            }

            #[wasm_bindgen]
            pub struct Foo {
                contents: u32,
            }
            
            #[wasm_bindgen]
            impl Foo {
                pub fn new() -> Foo {
                    Foo::with_contents(0)
                }
                pub fn with_contents(a: u32) -> Foo {
                    Foo { contents: a }
                }
                pub fn add(&mut self, amt: u32) -> u32 {
                    self.contents += amt;
                    self.contents
                }
            }
        "#)
        .file("test.js", r#"
            const assert = require('assert');
            const run = require('./out');

            var called = false;

            module.exports.hit = function() {
                called = true;
            };

            module.exports.test = function() {
                run();
                assert.strictEqual(called, true);
                var Foo = run.Foo;

                var r = Foo.new();
                assert.strictEqual(r.contents, 0);
                assert.strictEqual(r.add(0), 0);
                assert.strictEqual(r.add(1), 1);
                assert.strictEqual(r.add(2), 2);
                r.free();

                var r2 = Foo.with_contents(10);
                assert.strictEqual(r2.contents, 10);
                assert.strictEqual(r2.add(0), 0);
                assert.strictEqual(r2.add(1), 1);
                assert.strictEqual(r2.add(2), 2);
                r2.free();
            };

        "#)
        .test();
}
