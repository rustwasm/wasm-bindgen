#![allow(non_snake_case)]

use project;

#[test]
fn return_() {
    project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros)]

                extern crate wasm_bindgen;
                use wasm_bindgen::prelude::*;
                use wasm_bindgen::js;

                #[wasm_bindgen]
                pub fn gen_return(this: &js::Generator, value: &JsValue) -> JsValue {
                    this.return_(value)
                }
            "#,
        )
        .file(
            "test.js",
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
                #![feature(use_extern_macros)]

                extern crate wasm_bindgen;
                use wasm_bindgen::prelude::*;
                use wasm_bindgen::js;

                #[wasm_bindgen]
                pub fn next(this: &js::Generator, value: &JsValue) -> JsValue {
                    this.next(value)
                        .ok()
                        .expect("generator throws an error")
                }

                #[wasm_bindgen]
                pub fn next_throws_error(this: &js::Generator, value: &JsValue) -> bool {
                    this.next(value).is_err()
                }
            "#,
        )
        .file(
            "test.js",
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

                    function* brokenGenerator() {
                        throw new Error('Something went wrong');
                        yield 1;
                    }

                    assert.ok(wasm.next_throws_error(brokenGenerator(), undefined));
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
                #![feature(use_extern_macros)]

                extern crate wasm_bindgen;
                use wasm_bindgen::prelude::*;
                use wasm_bindgen::js;

                #[wasm_bindgen]
                pub fn gen_throws_error(this: &js::Generator, error: &js::Error) -> bool {
                    this.throw(error).is_err()
                }
            "#,
        )
        .file(
            "test.js",
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

                    assert.ok(wasm.gen_throws_error(gen, new Error('Something went wrong')));
                    assert.deepEqual(gen.next(), { value: undefined, done: true });
                }
            "#,
        )
        .test()
}
