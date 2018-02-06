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
                    fn doit(&self);
                }

                pub fn bar() {
                    Foo::create().doit();
                }
            }
        "#)
        .file("test.ts", r#"
            import * as wasm from "./out";
            import * as assert from "assert";

            let called = false;

            export class Foo {
                private random_property: string = '';

                static create() {
                    const ret = new Foo();
                    ret.random_property = 'this';
                    return ret;
                }

                doit() {
                    assert.strictEqual(this.random_property, 'this');
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
