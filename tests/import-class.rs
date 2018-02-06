extern crate test_support;

#[test]
fn simple() {
    test_support::project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            wasm_bindgen! {
                extern struct Math {
                    fn random() -> f64;
                    fn log(a: f64) -> f64;
                }

                pub fn get_random() -> f64 {
                    Math::random()
                }

                pub fn do_log(a: f64) -> f64 {
                    Math::log(a)
                }
            }
        "#)
        .file("test.ts", r#"
            import * as wasm from "./out";
            import * as assert from "assert";

            export function test() {
                wasm.get_random();
                assert.strictEqual(wasm.do_log(1.0), Math.log(1.0));
            }
        "#)
        .test();
}

#[test]
fn import_class() {
    test_support::project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            wasm_bindgen! {
                #[wasm_module = "./test"]
                extern struct Foo {
                    fn bar();
                }

                pub fn bar() {
                    Foo::bar();
                }
            }
        "#)
        .file("test.ts", r#"
            import * as wasm from "./out";
            import * as assert from "assert";

            let called = false;

            export class Foo {
                static bar() {
                    called = true;
                }
            }

            export function test() {
                wasm.bar();
                assert.strictEqual(called, true);
            }
        "#)
        .test();
}

#[test]
fn construct() {
    test_support::project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            wasm_bindgen! {
                #[wasm_module = "./test"]
                extern struct Foo {
                    fn create() -> Foo;
                    fn get_internal_string(&self) -> String;
                    fn append_to_internal_string(&self, s: &str);
                    fn assert_internal_string(&self, s: &str);
                }

                pub fn run() {
                    let f = Foo::create();
                    assert_eq!(f.get_internal_string(), "this");
                    f.append_to_internal_string(" foo");
                    f.assert_internal_string("this foo");
                }
            }
        "#)
        .file("test.ts", r#"
            import * as wasm from "./out";
            import * as assert from "assert";

            let called = false;

            export class Foo {
                private internal_string: string = '';

                static create() {
                    const ret = new Foo();
                    ret.internal_string = 'this';
                    return ret;
                }

                get_internal_string() {
                    return this.internal_string;
                }

                append_to_internal_string(s: string) {
                    this.internal_string += s;
                }

                assert_internal_string(s: string) {
                    assert.strictEqual(this.internal_string, s);
                    called = true;
                }
            }

            export function test() {
                wasm.run();
                assert.strictEqual(called, true);
            }
        "#)
        .test();
}
