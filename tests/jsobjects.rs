extern crate test_support;

#[test]
fn simple() {
    test_support::project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            wasm_bindgen! {
                extern "JS" {
                    fn foo(s: &JsObject);
                }
                pub fn bar(s: &JsObject) {
                    foo(s);
                }
            }
        "#)
        .file("test.ts", r#"
            import { Exports, Imports } from "./out";
            import * as assert from "assert";

            let ARG: string | null = null;

            export const imports: Imports = {
                foo(s) {
                    assert.strictEqual(ARG, null);
                    ARG = s;
                },
            };

            export function test(wasm: Exports) {
                assert.strictEqual(ARG, null);
                let sym = Symbol('test');
                wasm.bar(sym);
                assert.strictEqual(ARG, sym);
            }
        "#)
        .test();
}

#[test]
fn owned() {
    test_support::project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            wasm_bindgen! {
                extern "JS" {
                    fn foo(s: JsObject);
                }
                pub fn bar(s: JsObject) {
                    foo(s);
                }
            }
        "#)
        .file("test.ts", r#"
            import { Exports, Imports } from "./out";
            import * as assert from "assert";

            let ARG: Symbol | null = null;

            export const imports: Imports = {
                foo(s) {
                    assert.strictEqual(ARG, null);
                    ARG = s;
                },
            };

            export function test(wasm: Exports) {
                assert.strictEqual(ARG, null);
                let sym = Symbol('test');
                wasm.bar(sym);
                assert.strictEqual(ARG, sym);
            }
        "#)
        .test();
}

#[test]
fn clone() {
    test_support::project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            wasm_bindgen! {
                extern "JS" {
                    fn foo1(s: JsObject);
                    fn foo2(s: &JsObject);
                    fn foo3(s: JsObject);
                    fn foo4(s: &JsObject);
                    fn foo5(s: JsObject);
                }

                pub fn bar(s: JsObject) {
                    foo1(s.clone());
                    foo2(&s);
                    foo3(s.clone());
                    foo4(&s);
                    foo5(s);
                }
            }
        "#)
        .file("test.ts", r#"
            import { Exports, Imports } from "./out";
            import * as assert from "assert";

            let ARG = Symbol('test');

            export const imports: Imports = {
                foo1(s) { assert.strictEqual(s, ARG); },
                foo2(s) { assert.strictEqual(s, ARG); },
                foo3(s) { assert.strictEqual(s, ARG); },
                foo4(s) { assert.strictEqual(s, ARG); },
                foo5(s) { assert.strictEqual(s, ARG); },
            };

            export function test(wasm: Exports) {
                wasm.bar(ARG);
            }
        "#)
        .test();
}

#[test]
fn promote() {
    test_support::project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            wasm_bindgen! {
                extern "JS" {
                    fn foo1(s: &JsObject);
                    fn foo2(s: JsObject);
                    fn foo3(s: &JsObject);
                    fn foo4(s: JsObject);
                }

                pub fn bar(s: &JsObject) {
                    foo1(s);
                    foo2(s.clone());
                    foo3(s);
                    foo4(s.clone());
                }
            }
        "#)
        .file("test.ts", r#"
            import { Exports, Imports } from "./out";
            import * as assert from "assert";

            let ARG = Symbol('test');

            export const imports: Imports = {
                foo1(s) { assert.strictEqual(s, ARG); },
                foo2(s) { assert.strictEqual(s, ARG); },
                foo3(s) { assert.strictEqual(s, ARG); },
                foo4(s) { assert.strictEqual(s, ARG); },
            };

            export function test(wasm: Exports) {
                wasm.bar(ARG);
            }
        "#)
        .test();
}
