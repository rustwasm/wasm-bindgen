#![allow(non_snake_case)]

use project;

#[test]
fn keys() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn get_keys(this: &js::Array) -> js::ArrayIterator {
                this.keys()
            }

        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let numbers = [8, 5, 4, 3, 1, 2];
                let iterator = numbers.keys();
                let wasmIterator = wasm.get_keys(numbers);

                assert.equal(iterator.toString(), wasmIterator.toString());
                assert.equal(Array.from(iterator)[0], Array.from(wasmIterator)[0]);
            }
        "#,
        )
        .test()
}

#[test]
fn entries() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn get_entries(this: &js::Array) -> js::ArrayIterator {
                this.entries()
            }

        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let numbers = [8, 5, 4, 3, 1, 2];
                let iterator = numbers.entries();
                let wasmIterator = wasm.get_entries(numbers);
                let jsItem = iterator.next();
                let wasmItem = wasmIterator.next();

                assert.equal(iterator.toString(), wasmIterator.toString());
                assert.equal(jsItem.value[1], wasmItem.value[1]);
            }
        "#,
        )
        .test()
}

#[test]
fn values() {
    project()
        // Node.js does not have values()
        .headless(true)
        // Use a custom index.html so we can add a polyfill for Array#values for
        // older browsers.
        .file(
            "index.html",
            r#"
                <!DOCTYPE html>
                <html>
                    <body>
                        <div id="error"></div>
                        <div id="logs"></div>
                        <div id="status">incomplete</div>
                        <script>
                          Array.prototype.values = Array.prototype.values || function *() {
                            for (let i = 0; i < this.length; i++) {
                              yield this[i];
                            }
                          };
                        <script>
                        <script src="bundle.js"></script>
                    </body>
                </html>
            "#,
        )
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn get_values(this: &js::Array) -> js::ArrayIterator {
                this.values()
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let numbers = [8, 3, 2];
                let wasmIterator = wasm.get_values(numbers);

                assert.equal(wasmIterator.next().value, 8);
                assert.equal(wasmIterator.next().value, 3);
                assert.equal(wasmIterator.next().value, 2);
                assert.ok(wasmIterator.next().done);
            }
        "#,
        )
        .test()
}
