use std::fs::File;
use std::io::Read;

use super::project;

#[test]
fn simple() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section, wasm_import_module)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            #[wasm_bindgen(module = "./test")]
            extern {
                fn foo(s: &str);
                fn another(a: u32) -> i32;
                fn take_and_return_bool(a: bool) -> bool;
                fn return_object() -> JsValue;
            }

            #[wasm_bindgen]
            pub fn bar(s: &str) {
                foo(s);
            }

            #[wasm_bindgen]
            pub fn another_thunk(a: u32) -> i32 {
                another(a)
            }

            #[wasm_bindgen]
            pub fn bool_thunk(a: bool) -> bool {
                take_and_return_bool(a)
            }

            #[wasm_bindgen]
            pub fn get_the_object() -> JsValue {
                return_object()
            }
        "#,
        )
        .file(
            "test.ts",
            r#"
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
        "#,
        )
        .test();
}

#[test]
fn unused() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section, wasm_import_module)]
            #![allow(dead_code)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            #[wasm_bindgen(module = "./test")]
            extern {
                fn debug_print(s: &str);
            }

            #[wasm_bindgen]
            pub fn bar() {}
        "#,
        )
        .file(
            "test.ts",
            r#"
            import * as wasm from "./out";

            export function debug_print() {}

            export function test() {
                wasm.bar();
            }
        "#,
        )
        .test();
}

#[test]
fn string_ret() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section, wasm_import_module)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            #[wasm_bindgen(module = "./test")]
            extern {
                fn foo() -> String;
            }

            #[wasm_bindgen]
            pub fn run() {
                assert_eq!(foo(), "bar");
            }
        "#,
        )
        .file(
            "test.ts",
            r#"
            import * as wasm from "./out";

            export function foo(): string {
                return 'bar';
            }

            export function test() {
                wasm.run();
            }
        "#,
        )
        .test();
}

#[test]
fn strings() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section, wasm_import_module)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            #[wasm_bindgen(module = "./test")]
            extern {
                fn foo(a: String) -> String;
            }

            #[wasm_bindgen]
            pub fn bar(a: &str) -> String {
                foo(a.to_string())
            }

            #[wasm_bindgen]
            pub fn bar2(a: String) -> String {
                foo(a)
            }
        "#,
        )
        .file(
            "test.ts",
            r#"
            import * as wasm from "./out";
            import * as assert from "assert";

            export function foo(a: string): string {
                return a + 'b';
            }

            export function test() {
                assert.strictEqual(wasm.bar('a'), 'ab');
                assert.strictEqual(wasm.bar2('a'), 'ab');
            }
        "#,
        )
        .test();
}

#[test]
fn exceptions() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section, wasm_import_module)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            #[wasm_bindgen(module = "./test")]
            extern {
                fn foo();
                fn bar();
                #[wasm_bindgen(catch)]
                fn baz() -> Result<(), JsValue>;
            }

            #[wasm_bindgen]
            pub fn run() {
                foo();
                bar();
            }

            #[wasm_bindgen]
            pub fn run2() {
                assert!(baz().is_err());
                bar();
            }
        "#,
        )
        .file(
            "test.ts",
            r#"
            import { run, run2 } from "./out";
            import * as assert from "assert";

            let called = false;

            export function foo() {
                throw new Error('error!');
            }

            export function baz() {
                throw new Error('error2');
            }

            export function bar() {
                called = true;
            }

            export function test() {
                assert.throws(run, /error!/);
                assert.strictEqual(called, false);
                run2();
                assert.strictEqual(called, true);
            }
        "#,
        )
        .test();
}

#[test]
fn exn_caught() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section, wasm_import_module)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            #[wasm_bindgen(module = "./test")]
            extern {
                #[wasm_bindgen(catch)]
                fn foo() -> Result<(), JsValue>;
            }

            #[wasm_bindgen]
            pub fn run() -> JsValue {
                foo().unwrap_err()
            }
        "#,
        )
        .file(
            "test.ts",
            r#"
            import { run } from "./out";
            import * as assert from "assert";

            export function foo() {
                throw new Error('error!');
            }

            export function test() {
                const obj = run();
                assert.strictEqual(obj instanceof Error, true);
                assert.strictEqual(obj.message, 'error!');
            }
        "#,
        )
        .test();
}

#[test]
fn free_imports() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section, wasm_import_module)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            extern {
                fn parseInt(a: &str) -> u32;
            }

            #[wasm_bindgen]
            pub fn run() {
                assert_eq!(parseInt("3"), 3);
            }
        "#,
        )
        .file(
            "test.ts",
            r#"
            import { run } from "./out";

            export function test() {
                run();
            }
        "#,
        )
        .test();
}

#[test]
fn import_a_field() {
    project()
        .debug(false)
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section, wasm_import_module)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            #[wasm_bindgen(module = "./test")]
            extern {
                static IMPORT: JsValue;
            }

            #[wasm_bindgen]
            pub fn run() {
                assert_eq!(IMPORT.as_f64(), Some(1.0));
            }
        "#,
        )
        .file(
            "test.ts",
            r#"
            import { run } from "./out";

            export const IMPORT = 1.0;

            export function test() {
                run();
            }
        "#,
        )
        .test();
}

#[test]
fn rename() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section, wasm_import_module)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            #[wasm_bindgen(module = "./test")]
            extern {
                #[wasm_bindgen(js_name = baz)]
                fn foo();
            }

            #[wasm_bindgen]
            pub fn run() {
                foo();
            }
        "#,
        )
        .file(
            "test.ts",
            r#"
            import * as wasm from "./out";
            import * as assert from "assert";

            let called = false;

            export function baz() {
                called = true;
            }

            export function test() {
                wasm.run();
                assert.strictEqual(called, true);
            }
        "#,
        )
        .test();
}

#[test]
fn versions() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section, wasm_import_module)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            #[wasm_bindgen(module = "webpack", version = "^0.2.0")]
            extern {
                fn foo();
            }

            #[wasm_bindgen]
            pub fn run() {
                foo();
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            const fs = require("fs");
            const assert = require("assert");

            exports.test = function() {
                const bytes = fs.readFileSync('out_bg.wasm');
                const m = new WebAssembly.Module(bytes);
                const name = '__wasm_pack_unstable';
                const sections = WebAssembly.Module.customSections(m, name);
                assert.strictEqual(sections.length, 1);
                const b = new Uint8Array(sections[0]);
                const buf = new Buffer(b);
                const map = JSON.parse(buf.toString());
                assert.deepStrictEqual(map, {
                    version: '0.0.1',
                    modules: [
                        ['webpack', '^0.2.0']
                    ]
                });
            };
        "#,
        )
        .test();
}

#[test]
fn underscore_pattern() {
    project()
        .debug(false)
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section, wasm_import_module)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            #[wasm_bindgen(module = "./test")]
            extern {
                fn foo(_: u8);
            }

            #[wasm_bindgen]
            pub fn run() {
                foo(1);
            }
        "#)
        .file("test.ts", r#"
            import { run } from "./out";

            export function foo(_a: number) {
            }

            export function test() {
                run();
            }
        "#)
        .test();
}

#[test]
fn rust_keyword() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section, wasm_import_module)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            #[wasm_bindgen(module = "./test")]
            extern {
                #[wasm_bindgen(js_name = self)]
                fn foo() -> u32;
            }

            #[wasm_bindgen]
            pub fn run() {
                assert_eq!(foo(), 2);
            }
        "#,
        )
        .file(
            "test.ts",
            r#"
            import { run } from "./out";

            export function self() {
                return 2;
            }

            export function test() {
                run();
            }
        "#,
        )
        .test();
}

#[test]
fn rust_keyword2() {
    project()
        .debug(false)
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section, wasm_import_module)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            #[wasm_bindgen(module = "./test")]
            extern {
                pub type bar;
                #[wasm_bindgen(js_namespace = bar, js_name = foo)]
                static FOO: JsValue;
            }

            #[wasm_bindgen]
            pub fn run() {
                assert_eq!(FOO.as_f64(), Some(3.0));
            }
        "#,
        )
        .file(
            "test.ts",
            r#"
            import { run } from "./out";

            export const bar = {
                foo: 3,
            };

            export function test() {
                run();
            }
        "#,
        )
        .test();
}

#[test]
fn custom_type() {
    project()
        .debug(false)
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section, wasm_import_module)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            #[wasm_bindgen(module = "./test")]
            extern {
                fn foo(f: Foo) -> Foo;
                fn bad2() -> Foo;
            }

            #[wasm_bindgen]
            pub struct Foo(());

            #[wasm_bindgen]
            impl Foo {
                pub fn touch(&self) {
                    panic!()
                }
            }

            #[wasm_bindgen]
            pub fn run() {
                foo(Foo(()));
            }

            #[wasm_bindgen]
            pub fn bad() {
                bad2();
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import { run, Foo, bad } from "./out";

            let VAL: any = null;

            export function foo(f: Foo): Foo {
                VAL = f;
                return f;
            }

            export function bad2(): number {
                return 2;
            }

            export function test() {
                run();
                assert.throws(() => VAL.touch(), /Attempt to use a moved value/);
                assert.throws(bad, /expected value of type Foo/);
            }
        "#)
        .test();
}

#[test]
fn unused_imports_not_generated() {
    project()
        .debug(false)
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section, wasm_import_module)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            extern {
                pub fn foo();
            }

            #[wasm_bindgen]
            pub fn run() {
            }
        "#)
        .file("test.ts", r#"
            import { run } from "./out";

            export function test() {
                run();
            }
        "#)
        .test();

    let out = ::root().join("out.js");
    let mut contents = String::new();
    File::open(&out).unwrap()
        .read_to_string(&mut contents).unwrap();
    assert!(contents.contains("run"), "didn't find `run` in {}", contents);
    assert!(!contents.contains("foo"), "found `foo` in {}", contents);
}
