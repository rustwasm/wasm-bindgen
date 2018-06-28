#![allow(non_snake_case)]

use project;

#[test]
fn apply() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

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
            "test.ts",
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
fn length() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

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
            "test.ts",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.fn_length(() => {}), 0);
                assert.equal(wasm.fn_length((a: string) => console.log(a)), 1);
                assert.equal(wasm.fn_length((a: string, b: string) => console.log({ a, b })), 2);

                function fn0() {}
                function fn1(a: string) {
                    console.log(a);
                }
                function fn2(a: string, b: string) {
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
            #![feature(proc_macro, wasm_custom_section)]

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
            "test.ts",
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
            #![feature(proc_macro, wasm_custom_section)]

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
            "test.ts",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                function fn1(a: any, b: any) { return a + b }
                const fn2 = (a: number) => console.log(a);

                assert.equal(wasm.get_source_code(fn1), 'function fn1(a, b) { return a + b; }');
                assert.equal(wasm.get_source_code(fn2), 'function (a) { return console.log(a); }');
            }
        "#,
        )
        .test()
}
