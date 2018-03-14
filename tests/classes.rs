extern crate test_support;

#[test]
fn simple() {
    test_support::project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            pub struct Foo {
                contents: u32,
            }

            #[wasm_bindgen]
            impl Foo {
                pub fn new() -> Foo {
                    Foo::with_contents(0)
                }

                pub fn with_contents(a: u32) -> Foo {
                    Foo { contents: a }
                }

                pub fn add(&mut self, amt: u32) -> u32 {
                    self.contents += amt;
                    self.contents
                }
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import { Foo } from "./out";

            export function test() {
                const r = Foo.new();
                assert.strictEqual(r.add(0), 0);
                assert.strictEqual(r.add(1), 1);
                assert.strictEqual(r.add(1), 2);
                r.free();

                const r2 = Foo.with_contents(10);
                assert.strictEqual(r2.add(1), 11);
                assert.strictEqual(r2.add(2), 13);
                assert.strictEqual(r2.add(3), 16);
                r2.free();
            }
        "#)
        .test();
}

#[test]
fn strings() {
    test_support::project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            pub struct Foo {
                name: u32,
            }

            #[wasm_bindgen]
            pub struct Bar {
                contents: String,
            }

            #[wasm_bindgen]
            impl Foo {
                pub fn new() -> Foo {
                    Foo { name: 0 }
                }

                pub fn set(&mut self, amt: u32) {
                    self.name = amt;
                }

                pub fn bar(&self, mix: &str) -> Bar {
                    Bar { contents: format!("foo-{}-{}", mix, self.name) }
                }
            }

            #[wasm_bindgen]
            impl Bar {
                pub fn name(&self) -> String {
                    self.contents.clone()
                }
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import { Foo } from "./out";

            export function test() {
                const r = Foo.new();
                r.set(3);
                let bar = r.bar('baz');
                r.free();
                assert.strictEqual(bar.name(), "foo-baz-3");
                bar.free();
            }
        "#)
        .test();
}

#[test]
fn exceptions() {
    test_support::project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            pub struct A {
            }

            #[wasm_bindgen]
            impl A {
                pub fn new() -> A {
                    A {}
                }

                pub fn foo(&self, _: &A) {
                }

                pub fn bar(&mut self, _: &mut A) {
                }
            }

            #[wasm_bindgen]
            pub struct B {
            }

            #[wasm_bindgen]
            impl B {
                pub fn new() -> B {
                    B {}
                }
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import { A, B } from "./out";

            export function test() {
                assert.throws(() => new A(), /cannot invoke `new` directly/);
                let a = A.new();
                a.free();
                assert.throws(() => a.free(), /null pointer passed to rust/);

                let b = A.new();
                b.foo(b);
                assert.throws(() => b.bar(b), /recursive use of an object/);

                let c = A.new();
                let d = B.new();
                assert.throws(() => c.foo(d), /expected instance of A/);
                d.free();
                c.free();
            };
        "#)
        .file("test.d.ts", r#"
            export function test(): void;
        "#)
        .test();
}

#[test]
fn pass_one_to_another() {
    test_support::project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            pub struct A {}

            #[wasm_bindgen]
            impl A {
                pub fn new() -> A {
                    A {}
                }

                pub fn foo(&self, _other: &B) {
                }

                pub fn bar(&self, _other: B) {
                }
            }

            #[wasm_bindgen]
            pub struct B {}

            #[wasm_bindgen]
            impl B {
                pub fn new() -> B {
                    B {}
                }
            }
        "#)
        .file("test.ts", r#"
            import { A, B } from "./out";

            export function test() {
                let a = A.new();
                let b = B.new();
                a.foo(b);
                a.bar(b);
                a.free();
            }
        "#)
        .test();
}

#[test]
fn pass_into_js() {
    test_support::project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section, wasm_import_module)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            pub struct Foo(i32);

            #[wasm_bindgen]
            impl Foo {
                pub fn inner(&self) -> i32 {
                    self.0
                }
            }

            #[wasm_bindgen(module = "./test")]
            extern {
                fn take_foo(foo: Foo);
            }

            #[wasm_bindgen]
            pub fn run() {
                take_foo(Foo(13));
            }
        "#)
        .file("test.ts", r#"
            import { run, Foo } from "./out";
            import * as assert from "assert";

            export function take_foo(foo: Foo) {
                assert.strictEqual(foo.inner(), 13);
                foo.free();
                assert.throws(() => foo.free(), /null pointer passed to rust/);
            }

            export function test() {
                run();
            }
        "#)
        .test();
}

#[test]
fn issue_27() {
    test_support::project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            pub struct Context {}

            #[wasm_bindgen]
            impl Context {
                pub fn parse(&self, expr: &str) -> Expr {
                    panic!()
                }
                pub fn eval(&self, expr: &Expr) -> f64 {
                    panic!()
                }
                pub fn set(&mut self, var: &str, val: f64) {
                    panic!()
                }
            }

            #[wasm_bindgen]
            pub struct Expr {}

            #[wasm_bindgen]
            pub fn context() -> Context {
                Context {}
            }
        "#)
        .file("test.ts", r#"
            import { context } from "./out";

            export function test() {
                context();
            }
        "#)
        .test();
}
