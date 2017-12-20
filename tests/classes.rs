extern crate test_support;

#[test]
fn simple() {
    test_support::project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            wasm_bindgen! {
                pub struct Foo {
                    contents: u32,
                }

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
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import { Exports, Imports } from "./out";

            export const imports: Imports = {};

            export function test(wasm: Exports) {
                const r = wasm.Foo.new();
                assert.strictEqual(r.add(0), 0);
                assert.strictEqual(r.add(1), 1);
                assert.strictEqual(r.add(1), 2);
                r.free();

                const r2 = wasm.Foo.with_contents(10);
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
            #![feature(proc_macro)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            wasm_bindgen! {
                pub struct Foo {
                    name: u32,
                }

                pub struct Bar {
                    contents: String,
                }

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

                impl Bar {
                    pub fn name(&self) -> String {
                        self.contents.clone()
                    }
                }
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import { Exports, Imports } from "./out";

            export const imports: Imports = {};

            export function test(wasm: Exports) {
                const r = wasm.Foo.new();
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
            #![feature(proc_macro)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            wasm_bindgen! {
                pub struct A {
                }

                impl A {
                    pub fn new() -> A {
                        A {}
                    }

                    pub fn foo(&self, _: &A) {
                    }

                    pub fn bar(&mut self, _: &mut A) {
                    }
                }

                pub struct B {
                }

                impl B {
                    pub fn new() -> B {
                        B {}
                    }
                }
            }
        "#)
        .file("test.js", r#"
            var assert = require("assert");

            exports.imports = {};
            exports.test = function(wasm) {
                assert.throws(() => new wasm.A(), /cannot invoke `new` directly/);
                let a = wasm.A.new();
                a.free();
                // TODO: figure out a better error message?
                assert.throws(() => a.free(), /RuntimeError: unreachable/);

                let b = wasm.A.new();
                b.foo(b);
                assert.throws(() => b.bar(b), /RuntimeError: unreachable/);

                let c = wasm.A.new();
                let d = wasm.B.new();
                assert.throws(() => c.foo(d), /expected instance of A/);
                d.free();
                c.free();
            };
        "#)
        .file("test.d.ts", r#"
            import { Exports, Imports } from "./out";

            export const imports: Imports;

            export function test(wasm: Exports): void;
        "#)
        .test();
}

#[test]
fn pass_one_to_another() {
    test_support::project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            wasm_bindgen! {
                pub struct A {}

                impl A {
                    pub fn new() -> A {
                        A {}
                    }

                    pub fn foo(&self, _other: &B) {
                    }

                    pub fn bar(&self, _other: B) {
                    }
                }

                pub struct B {}

                impl B {
                    pub fn new() -> B {
                        B {}
                    }
                }
            }
        "#)
        .file("test.ts", r#"
            import { Exports, Imports } from "./out";

            export const imports: Imports = {};

            export function test(wasm: Exports) {
                let a = wasm.A.new();
                let b = wasm.B.new();
                a.foo(b);
                a.bar(b);
                a.free();
            }
        "#)
        .test();
}
