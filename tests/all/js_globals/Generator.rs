#![allow(non_snake_case)]

use project;

#[test]
fn gen_return() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn gen_return(this: &js::Generator, value: &JsValue) -> JsValue {
                this.gen_return(value)
            }
        "#,
        )
        .file(
            "test.ts",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                function* generator() {
                    yield 1;
                    yield 2;
                }

                const gen = generator();
                gen.next();

                const res = wasm.gen_return(gen, 42);
                assert.deepEqual(res, { value: 42, done: true });

                const next = gen.next();
                assert.deepEqual(next, { value: undefined, done: true });
            }
        "#,
        )
        .test()
}

#[test]
fn next() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn next(this: &js::Generator, value: &JsValue) -> JsValue {
                this.next(value)
            }
        "#,
        )
        .file(
            "test.ts",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                function* generator() {
                    const reply = yield '2 * 2';

                    return reply === 4;
                }

                const gen = generator();

                const q = wasm.next(gen, undefined);
                assert.deepEqual(q, { value: '2 * 2', done: false });

                const a = wasm.next(gen, 4);
                assert.deepEqual(a, { value: true, done: true });
            }
        "#,
        )
        .test()
}

#[test]
fn throw() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn gen_throw(this: &js::Generator, error: &js::Error) -> JsValue {
                this.throw(error)
            }
        "#,
        )
        .file(
            "test.ts",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                function* generator() {
                    yield 1;
                    yield 2;
                }

                const gen = generator();
                gen.next();

                try {
                    wasm.gen_throw(gen, new Error('Something went wrong'));
                } catch(err) {
                    assert.equal(err.message, 'Something went wrong');
                }

                assert.deepEqual(gen.next(), { value: undefined, done: true });
            }
        "#,
        )
        .test()
}
