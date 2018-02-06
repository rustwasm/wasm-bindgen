extern crate test_support;

#[test]
fn simple() {
    test_support::project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            wasm_bindgen! {
                #[wasm_module = "./test"]
                extern "JS" {
                    fn foo(s: &str);
                    fn another(a: u32) -> i32;
                    fn take_and_return_bool(a: bool) -> bool;
                    fn return_object() -> JsValue;
                }
                pub fn bar(s: &str) {
                    foo(s);
                }
                pub fn another_thunk(a: u32) -> i32 {
                    another(a)
                }
                pub fn bool_thunk(a: bool) -> bool {
                    take_and_return_bool(a)
                }

                pub fn get_the_object() -> JsValue {
                    return_object()
                }
            }
        "#)
        .file("test.ts", r#"
            import * as wasm from "./out";
            import * as assert from "assert";

            let ARG: string | null = null;
            let ANOTHER_ARG: number | null = null;
            let SYM = (Symbol as any)('a');

            export function foo(s: string): void {
                assert.strictEqual(ARG, null);
                assert.strictEqual(s, "foo");
                ARG = s;
            }
            export function another(s: number): number {
                assert.strictEqual(ANOTHER_ARG, null);
                assert.strictEqual(s, 21);
                ANOTHER_ARG = s;
                return 35;
            }
            export function take_and_return_bool(s: boolean): boolean {
                return s;
            }
            export function return_object(): any {
                return SYM;
            }

            export function test() {
                assert.strictEqual(ARG, null);
                wasm.bar("foo");
                assert.strictEqual(ARG, "foo");

                assert.strictEqual(ANOTHER_ARG, null);
                assert.strictEqual(wasm.another_thunk(21), 35);
                assert.strictEqual(ANOTHER_ARG, 21);

                assert.strictEqual(wasm.bool_thunk(true), true);
                assert.strictEqual(wasm.bool_thunk(false), false);

                assert.strictEqual(wasm.get_the_object(), SYM);
            }
        "#)
        .test();
}

#[test]
fn unused() {
    test_support::project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            wasm_bindgen! {
                #[wasm_module = "./test"]
                extern "JS" {
                    fn debug_print(s: &str);
                }

                pub fn bar() {}
            }
        "#)
        .file("test.ts", r#"
            import * as wasm from "./out";

            export function debug_print() {}

            export function test() {
                wasm.bar();
            }
        "#)
        .test();
}

#[test]
fn strings() {
    test_support::project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            wasm_bindgen! {
                #[wasm_module = "./test"]
                extern "JS" {
                    fn foo(a: String) -> String;
                }

                pub fn bar(a: &str) -> String {
                    foo(a.to_string())
                }

                pub fn bar2(a: String) -> String {
                    foo(a)
                }
            }
        "#)
        .file("test.ts", r#"
            import * as wasm from "./out";
            import * as assert from "assert";

            export function foo(a: string): string {
                return a + 'b';
            }

            export function test() {
                assert.strictEqual(wasm.bar('a'), 'ab');
                assert.strictEqual(wasm.bar2('a'), 'ab');
            }
        "#)
        .test();
}
