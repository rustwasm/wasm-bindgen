#![allow(non_snake_case)]

use project;

#[test]
fn apply() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn apply(this: &js::Function, context: &JsValue, args: &js::Array) -> js::Function {
                this.apply(context, args)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.apply(Math.max, {}, [1, 2, 3]), 3);

                const arr = [1, 2];
                wasm.apply(Array.prototype.push, arr, [3]);
                assert.equal(arr[2], 3);
            }
        "#,
        )
        .test()
}

#[test]
fn bind() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn bind(this: &js::Function, context: &JsValue) -> js::Function {
                this.bind(context)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                const obj = {
                    a: 0,
                    fn: function () {
                        return this.a + 1;
                    }
                }

                const boundFn = wasm.bind(obj.fn, { a: 41 });
                assert.equal(boundFn(), 42);
            }
        "#,
        )
        .test()
}

#[test]
fn length() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn fn_length(this: &js::Function) -> u32 {
                this.length()
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.fn_length(() => {}), 0);
                assert.equal(wasm.fn_length(a => console.log(a)), 1);
                assert.equal(wasm.fn_length((a, b) => console.log({ a, b })), 2);

                function fn0() {}
                function fn1(a) {
                    console.log(a);
                }
                function fn2(a, b) {
                    console.log({ a, b });
                }

                assert.equal(wasm.fn_length(fn0), 0);
                assert.equal(wasm.fn_length(fn1), 1);
                assert.equal(wasm.fn_length(fn2), 2);
            }
        "#,
        )
        .test()
}

#[test]
fn name() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn fn_name(this: &js::Function) -> js::JsString {
                this.name()
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                function namedFn() {}
                assert.equal(wasm.fn_name(namedFn), 'namedFn');

                assert.equal(wasm.fn_name(namedFn.bind({})), 'bound namedFn');

                const obj = {
                    method: () => {}
                }
                assert.equal(wasm.fn_name(obj.method), 'method');

                assert.equal(wasm.fn_name(new Function()), 'anonymous');

                assert.equal(wasm.fn_name(() => {}), '');

                const closure = () => {};
                assert.equal(wasm.fn_name(closure), 'closure');
            }
        "#,
        )
        .test()
}

#[test]
fn to_string() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn get_source_code(this: &js::Function) -> js::JsString {
                this.to_string()
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                function fn1(a, b) { return a + b; }
                const fn2 = a => console.log(a);

                assert.equal(wasm.get_source_code(fn1), 'function fn1(a, b) { return a + b; }');
                assert.equal(wasm.get_source_code(fn2), 'a => console.log(a)');
            }
        "#,
        )
        .test()
}
