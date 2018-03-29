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
            };
        "#)
        .test();
}
