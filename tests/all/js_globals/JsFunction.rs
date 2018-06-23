#![allow(non_snake_case)]

use project;

#[test]
fn length() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn fn_length(this: &js::JsFunction) -> u32 {
                this.length()
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(0, wasm.fn_length(() => {}));
                assert.equal(1, wasm.fn_length((a: string) => console.log(a)));
                assert.equal(2, wasm.fn_length((a: string, b: string) => console.log({ a, b })));

                function fn0() {}
                function fn1(a: string) {
                    console.log(a);
                }
                function fn2(a: string, b: string) {
                    console.log({ a, b });
                }

                assert.equal(0, wasm.fn_length(fn0));
                assert.equal(1, wasm.fn_length(fn1));
                assert.equal(2, wasm.fn_length(fn2));
            }
        "#)
        .test()
}

#[test]
fn name() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn fn_name(this: &js::JsFunction) -> js::JsString {
                this.name()
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                function namedFn() {}
                assert.equal('namedFn', wasm.fn_name(namedFn));

                assert.equal('bound namedFn', wasm.fn_name(namedFn.bind({})));

                const obj = {
                    method: () => {}
                }
                assert.equal('method', wasm.fn_name(obj.method));

                assert.equal('anonymous', wasm.fn_name(new Function()));

                assert.equal('', wasm.fn_name(() => {}));

                const closure = () => {};
                assert.equal('closure', wasm.fn_name(closure));
            }
        "#)
        .test()
}
