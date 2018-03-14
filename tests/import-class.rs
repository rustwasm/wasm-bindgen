extern crate test_support;

#[test]
fn simple() {
    test_support::project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section, wasm_import_module)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            pub fn get_random() -> f64 {
                random()
            }

            #[wasm_bindgen]
            pub fn do_log(a: f64) -> f64 {
                log(a)
            }

            #[wasm_bindgen]
            extern {
                #[wasm_bindgen(js_namespace = Math)]
                fn random() -> f64;
                #[wasm_bindgen(js_namespace = Math)]
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
            #![feature(proc_macro, wasm_custom_section, wasm_import_module)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            #[wasm_bindgen(module = "./another")]
            extern {
                #[wasm_bindgen(js_namespace = Foo)]
                fn bar();
            }

            #[wasm_bindgen]
            pub fn baz() {
                bar();
            }
        "#)
        .file("test.ts", r#"
            import { baz } from "./out";
            import { called } from "./another";
            import * as assert from "assert";

            export function test() {
                baz();
                assert.strictEqual(called, true);
            }
        "#)
        .file("another.ts", r#"
            export let called = false;

            export class Foo {
                static bar() {
                    called = true;
                }
            }
        "#)
        .test();
}

#[test]
fn construct() {
    test_support::project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section, wasm_import_module)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            #[wasm_bindgen(module = "./another")]
            extern {
                type Foo;
                #[wasm_bindgen(js_namespace = Foo)]
                fn create() -> Foo;
                #[wasm_bindgen(method)]
                fn get_internal_string(this: &Foo) -> String;
                #[wasm_bindgen(method)]
                fn append_to_internal_string(this: &Foo, s: &str);
                #[wasm_bindgen(method)]
                fn assert_internal_string(this: &Foo, s: &str);
            }

            #[wasm_bindgen]
            pub fn run() {
                let f = Foo::create();
                assert_eq!(f.get_internal_string(), "this");
                f.append_to_internal_string(" foo");
                f.assert_internal_string("this foo");
            }
        "#)
        .file("test.ts", r#"
            import { run } from "./out";
            import { called } from "./another";
            import * as assert from "assert";

            export function test() {
                run();
                assert.strictEqual(called, true);
            }
        "#)
        .file("another.ts", r#"
            import * as assert from "assert";

            export let called = false;

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
        "#)
        .test();
}

#[test]
fn new_constructors() {
    test_support::project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section, wasm_import_module)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            #[wasm_bindgen(module = "./another")]
            extern {
                type Foo;
                #[wasm_bindgen(constructor)]
                fn new(arg: i32) -> Foo;
                #[wasm_bindgen(method)]
                fn get(this: &Foo) -> i32;
            }

            #[wasm_bindgen]
            pub fn run() {
                let f = Foo::new(1);
                assert_eq!(f.get(), 2);
            }
        "#)
        .file("test.ts", r#"
            import { run } from "./out";

            export function test() {
                run();
            }
        "#)
        .file("another.ts", r#"
            export class Foo {
                constructor(private field: number) {
                }

                get() {
                    return this.field + 1;
                }
            }
        "#)
        .test();
}

#[test]
fn switch_methods() {
    test_support::project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section, wasm_import_module)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            #[wasm_bindgen(module = "./another")]
            extern {
                type Foo;

                #[wasm_bindgen(constructor)]
                fn new() -> Foo;

                #[wasm_bindgen(js_namespace = Foo)]
                fn a();

                #[wasm_bindgen(method)]
                fn b(this: &Foo);
            }

            #[wasm_bindgen]
            pub fn a() {
                Foo::a();
            }

            #[wasm_bindgen]
            pub fn b() {
                Foo::new().b();
            }
        "#)
        .file("test.ts", r#"
            import { a, b } from "./out";
            import { Foo, called } from "./another";
            import * as assert from "assert";

            export function test() {
                assert.strictEqual(called.a, false);
                a();
                assert.strictEqual(called.a, true);
                called.a = false;
                Foo.a = function() {};
                assert.strictEqual(called.a, false);
                a();
                assert.strictEqual(called.a, true);

                called.a = false;
                assert.strictEqual(called.a, false);
                b();
                assert.strictEqual(called.a, true);
                called.a = false;
                Foo.prototype.b = function() {};
                assert.strictEqual(called.a, false);
                b();
                assert.strictEqual(called.a, true);
            }
        "#)
        .file("another.ts", r#"
            export let called = { a: false };

            export class Foo {
                constructor() {
                }

                static a() {
                    called.a = true;
                }

                b() {
                    called.a = true;
                }
            }
        "#)
        .test();
}

#[test]
fn properties() {
    test_support::project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section, wasm_import_module)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            #[wasm_bindgen(module = "./another")]
            extern {
                type Foo;

                #[wasm_bindgen(constructor)]
                fn new() -> Foo;

                #[wasm_bindgen(getter, method)]
                fn a(this: &Foo) -> i32;

                #[wasm_bindgen(setter, method)]
                fn set_a(this: &Foo, a: i32);
            }

            #[wasm_bindgen]
            pub fn run() {
                let a = Foo::new();
                assert_eq!(a.a(), 1);
                a.set_a(2);
                assert_eq!(a.a(), 2);
            }
        "#)
        .file("test.ts", r#"
            import { run } from "./out";

            export function test() {
                run();
            }
        "#)
        .file("another.ts", r#"
            export class Foo {
                constructor(private num: number) {
                    this.num = 1;
                }

                get a() {
                    return this.num;
                }

                set a(val) {
                    this.num = val;
                }
            }
        "#)
        .test();
}

#[test]
fn rename_setter_getter() {
    test_support::project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section, wasm_import_module)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            #[wasm_bindgen(module = "./another")]
            extern {
                type Foo;

                #[wasm_bindgen(constructor)]
                fn new() -> Foo;

                #[wasm_bindgen(getter = a, method)]
                fn test(this: &Foo) -> i32;

                #[wasm_bindgen(setter = a, method)]
                fn another(this: &Foo, a: i32);
            }

            #[wasm_bindgen]
            pub fn run() {
                let a = Foo::new();
                assert_eq!(a.test(), 1);
                a.another(2);
                assert_eq!(a.test(), 2);
            }
        "#)
        .file("test.ts", r#"
            import { run } from "./out";

            export function test() {
                run();
            }
        "#)
        .file("another.ts", r#"
            export class Foo {
                constructor(private num: number) {
                    this.num = 1;
                }

                get a() {
                    return this.num;
                }

                set a(val) {
                    this.num = val;
                }
            }
        "#)
        .test();
}
