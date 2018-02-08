extern crate test_support;

#[test]
fn simple() {
    test_support::project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            #[no_mangle]
            pub extern fn get_random() -> f64 {
                Math::random()
            }

            #[wasm_bindgen]
            #[no_mangle]
            pub extern fn do_log(a: f64) -> f64 {
                Math::log(a)
            }

            #[wasm_bindgen]
            extern {
                type Math;
                #[wasm_bindgen(static = Math)]
                fn random() -> f64;
                #[wasm_bindgen(static = Math)]
                fn log(a: f64) -> f64;
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

            #[wasm_bindgen(module = "./test")]
            extern {
                type Foo;
                #[wasm_bindgen(static = Foo)]
                fn bar();
            }

            #[wasm_bindgen]
            #[no_mangle]
            pub extern fn bar() {
                Foo::bar();
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

            #[wasm_bindgen(module = "./test")]
            extern {
                type Foo;
                #[wasm_bindgen(static = Foo)]
                fn create() -> Foo;
                #[wasm_bindgen(method)]
                fn get_internal_string(this: &Foo) -> String;
                #[wasm_bindgen(method)]
                fn append_to_internal_string(this: &Foo, s: &str);
                #[wasm_bindgen(method)]
                fn assert_internal_string(this: &Foo, s: &str);
            }

            #[wasm_bindgen]
            #[no_mangle]
            pub extern fn run() {
                let f = Foo::create();
                assert_eq!(f.get_internal_string(), "this");
                f.append_to_internal_string(" foo");
                f.assert_internal_string("this foo");
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

#[test]
fn new_constructors() {
    test_support::project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            #[wasm_bindgen(module = "./test")]
            extern {
                type Foo;
                #[wasm_bindgen(constructor)]
                fn new(arg: i32) -> Foo;
                #[wasm_bindgen(method)]
                fn get(this: &Foo) -> i32;
            }

            #[wasm_bindgen]
            #[no_mangle]
            pub extern fn run() {
                let f = Foo::new(1);
                assert_eq!(f.get(), 2);
            }
        "#)
        .file("test.ts", r#"
            import { run } from "./out";

            export class Foo {
                constructor(private field: number) {
                }

                get() {
                    return this.field + 1;
                }
            }

            export function test() {
                run();
            }
        "#)
        .test();
}
