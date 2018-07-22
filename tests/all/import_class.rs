use super::project;

#[test]
fn simple() {
    project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros)]

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
            "#,
        )
        .file(
            "test.js",
            r#"
                import * as wasm from "./out";
                import * as assert from "assert";

                export function test() {
                    wasm.get_random();
                    assert.strictEqual(wasm.do_log(1.0), Math.log(1.0));
                }
            "#,
        )
        .test();
}

#[test]
fn import_class() {
    project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros)]

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
            "#,
        )
        .file(
            "test.js",
            r#"
                import { baz } from "./out";
                import { called } from "./another";
                import * as assert from "assert";

                export function test() {
                    baz();
                    assert.strictEqual(called, true);
                }
            "#,
        )
        .file(
            "another.js",
            r#"
                export let called = false;

                export class Foo {
                    static bar() {
                        called = true;
                    }
                }
            "#,
        )
        .test();
}

#[test]
fn construct() {
    project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros)]

                extern crate wasm_bindgen;

                use wasm_bindgen::prelude::*;

                #[wasm_bindgen(module = "./another")]
                extern {
                    #[derive(Clone)]
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
                    assert_eq!(f.clone().get_internal_string(), "this");
                    f.append_to_internal_string(" foo");
                    f.assert_internal_string("this foo");
                }
            "#,
        )
        .file(
            "test.js",
            r#"
                import { run } from "./out";
                import { called } from "./another";
                import * as assert from "assert";

                export function test() {
                    run();
                    assert.strictEqual(called, true);
                }
            "#,
        )
        .file(
            "another.js",
            r#"
                import * as assert from "assert";

                export let called = false;

                export class Foo {
                    static create() {
                        const ret = new Foo();
                        ret.internal_string = 'this';
                        return ret;
                    }

                    get_internal_string() {
                        return this.internal_string;
                    }

                    append_to_internal_string(s) {
                        this.internal_string += s;
                    }

                    assert_internal_string(s) {
                        assert.strictEqual(this.internal_string, s);
                        called = true;
                    }
                }

                Foo.internal_string = '';
            "#,
        )
        .test();
}

#[test]
fn new_constructors() {
    project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros)]

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
            "#,
        )
        .file(
            "test.js",
            r#"
                import { run } from "./out";

                export function test() {
                    run();
                }
            "#,
        )
        .file(
            "another.js",
            r#"
                export class Foo {
                    constructor(field) {
                        this.field = field;
                    }

                    get() {
                        return this.field + 1;
                    }
                }
            "#,
        )
        .test();
}

#[test]
fn switch_methods() {
    project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros)]

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
            "#,
        )
        .file(
            "test.js",
            r#"
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
            "#,
        )
        .file(
            "another.js",
            r#"
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
            "#,
        )
        .test();
}

#[test]
fn properties() {
    project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros)]

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
            "#,
        )
        .file(
            "test.js",
            r#"
                import { run } from "./out";

                export function test() {
                    run();
                }
            "#,
        )
        .file(
            "another.js",
            r#"
                export class Foo {
                    constructor() {
                        this.num = 1;
                    }

                    get a() {
                        return this.num;
                    }

                    set a(val) {
                        this.num = val;
                    }
                }
            "#,
        )
        .test();
}

#[test]
fn rename_setter_getter() {
    project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros)]

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
                    let x: fn() -> Foo = Foo::new;
                    let a = x();
                    assert_eq!(a.test(), 1);
                    a.another(2);
                    assert_eq!(a.test(), 2);
                }
            "#,
        )
        .file(
            "test.js",
            r#"
                import { run } from "./out";

                export function test() {
                    run();
                }
            "#,
        )
        .file(
            "another.js",
            r#"
                export class Foo {
                    constructor() {
                        this.num = 1;
                    }

                    get a() {
                        return this.num;
                    }

                    set a(val) {
                        this.num = val;
                    }
                }
            "#,
        )
        .test();
}

#[test]
fn deny_missing_docs() {
    project()
        .file(
            "src/lib.rs",
            r#"
                //! dox
                #![feature(use_extern_macros)]
                #![deny(missing_docs)]
                #![allow(dead_code)]

                extern crate wasm_bindgen;

                use wasm_bindgen::prelude::*;

                /// dox
                #[wasm_bindgen]
                pub struct Bar {
                    /// dox
                    pub a: u32,
                    b: i64,
                }

                #[wasm_bindgen]
                extern {
                    /// dox
                    pub type Foo;

                    /// dox
                    #[wasm_bindgen(constructor)]
                    pub fn new() -> Foo;

                    /// dox
                    #[wasm_bindgen(getter = a, method)]
                    pub fn test(this: &Foo) -> i32;

                    /// dox
                    pub fn foo();
                }

                /// dox
                #[wasm_bindgen]
                pub fn test() {
                }
            "#,
        )
        .test();
}

#[test]
fn options() {
    project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros)]

                extern crate wasm_bindgen;

                use wasm_bindgen::prelude::*;

                #[wasm_bindgen(module = "./foo")]
                extern {
                    pub type Foo;
                    #[wasm_bindgen(constructor)]
                    fn new() -> Foo;

                    fn take_none(val: Option<Foo>);
                    fn take_some(val: Option<Foo>);
                    fn return_null() -> Option<Foo>;
                    fn return_undefined() -> Option<Foo>;
                    fn return_some() -> Option<Foo>;
                    fn run_rust_tests();
                }

                #[wasm_bindgen]
                pub fn test() {
                    take_none(None);
                    take_some(Some(Foo::new()));
                    assert!(return_null().is_none());
                    assert!(return_undefined().is_none());
                    assert!(return_some().is_some());
                    run_rust_tests();
                }

                #[wasm_bindgen]
                pub fn rust_take_none(a: Option<Foo>) {
                    assert!(a.is_none());
                }

                #[wasm_bindgen]
                pub fn rust_take_some(a: Option<Foo>) {
                    assert!(a.is_some());
                }

                #[wasm_bindgen]
                pub fn rust_return_none() -> Option<Foo> {
                    None
                }

                #[wasm_bindgen]
                pub fn rust_return_some() -> Option<Foo> {
                    Some(Foo::new())
                }
            "#,
        )
        .file(
            "foo.js",
            r#"
                import { strictEqual } from "assert";
                import * as wasm from "./out";

                export class Foo {
                }

                export function take_none(val) {
                    strictEqual(val, undefined);
                }

                export function take_some(val) {
                    strictEqual(val === undefined, false);
                }

                export function return_null() {
                    return null;
                }

                export function return_undefined() {
                    return undefined;
                }

                export function return_some() {
                    return new Foo();
                }

                export function run_rust_tests() {
                    wasm.rust_take_none();
                    wasm.rust_take_none(null);
                    wasm.rust_take_none(undefined);
                    wasm.rust_take_some(new Foo());
                    strictEqual(wasm.rust_return_none(), undefined);
                    strictEqual(wasm.rust_return_none(), undefined);
                    strictEqual(wasm.rust_return_some() === undefined, false);
                }
            "#,
        )
        .test();
}
