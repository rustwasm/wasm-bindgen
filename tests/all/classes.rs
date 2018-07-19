use super::project;

#[test]
fn simple() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros, wasm_import_module)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            pub struct Foo {
                contents: u32,
            }

            #[wasm_bindgen]
            impl Foo {
                #[wasm_bindgen(constructor)]
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

                pub fn consume(self) -> u32 {
                    self.contents
                }
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import { Foo } from "./out";

            export function test() {
                const r = Foo.new();
                assert.strictEqual(r.add(0), 0);
                assert.strictEqual(r.add(1), 1);
                assert.strictEqual(r.add(1), 2);
                r.add(2);
                assert.strictEqual(r.consume(), 4);
                assert.throws(() => r.free(), /null pointer passed to rust/);

                const r2 = Foo.with_contents(10);
                assert.strictEqual(r2.add(1), 11);
                assert.strictEqual(r2.add(2), 13);
                assert.strictEqual(r2.add(3), 16);
                r2.free();

                const r3 = new Foo();
                assert.strictEqual(r3.add(42), 42);
                r3.free();
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
            #![feature(use_extern_macros, wasm_import_module)]

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
        "#,
        )
        .file(
            "test.js",
            r#"
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
                #![feature(use_extern_macros, wasm_import_module)]

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
            "#,
        )
        .file(
            "test.js",
            r#"
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
            "#,
        )
        .test();
}

#[test]
fn pass_one_to_another() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros, wasm_import_module)]

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
        "#,
        )
        .file(
            "test.js",
            r#"
            import { A, B } from "./out";

            export function test() {
                let a = A.new();
                let b = B.new();
                a.foo(b);
                a.bar(b);
                a.free();
            }
        "#,
        )
        .test();
}

#[test]
fn pass_into_js() {
    project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros, wasm_import_module)]

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
            "#,
        )
        .file(
            "test.js",
            r#"
                import { run, Foo } from "./out";
                import * as assert from "assert";

                export function take_foo(foo) {
                    assert.strictEqual(foo.inner(), 13);
                    foo.free();
                    assert.throws(() => foo.free(), /null pointer passed to rust/);
                }

                export function test() {
                    run();
                }
            "#,
        )
        .test();
}

#[test]
fn issue_27() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros, wasm_import_module)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            pub struct Context {}

            #[wasm_bindgen]
            impl Context {
                pub fn parse(&self, _expr: &str) -> Expr {
                    panic!()
                }
                pub fn eval(&self, _expr: &Expr) -> f64 {
                    panic!()
                }
                pub fn set(&mut self, _var: &str, _val: f64) {
                    panic!()
                }
            }

            #[wasm_bindgen]
            pub struct Expr {}

            #[wasm_bindgen]
            pub fn context() -> Context {
                Context {}
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import { context } from "./out";

            export function test() {
                context();
            }
        "#,
        )
        .test();
}

#[test]
fn pass_into_js_as_js_class() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros, wasm_import_module)]

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
                fn take_foo(foo: JsValue);
            }

            #[wasm_bindgen]
            pub fn run() {
                take_foo(Foo(13).into());
            }
        "#,
        )
        .file(
            "test.js",
            r#"
                import { run, Foo } from "./out";
                import * as assert from "assert";

                export function take_foo(foo) {
                    assert.ok(foo instanceof Foo);
                    assert.strictEqual(foo.inner(), 13);
                    foo.free();
                }

                export function test() {
                    run();
                }
            "#,
        )
        .test();
}

#[test]
fn constructors() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros, wasm_import_module)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            pub fn cross_item_construction() -> Bar {
                Bar::other_name(7, 8)
            }

            #[wasm_bindgen]
            pub struct Foo {
                number: u32,
            }

            #[wasm_bindgen]
            impl Foo {
                #[wasm_bindgen(constructor)]
                pub fn new(number: u32) -> Foo {
                    Foo { number }
                }

                pub fn get_number(&self) -> u32 {
                    self.number
                }
            }

            #[wasm_bindgen]
            pub struct Bar {
                number: u32,
                number2: u32,
            }

            #[wasm_bindgen]
            impl Bar {
                #[wasm_bindgen(constructor)]
                pub fn other_name(number: u32, number2: u32) -> Bar {
                    Bar { number, number2 }
                }

                pub fn get_sum(&self) -> u32 {
                    self.number + self.number2
                }
            }
        "#,
        )
        .file(
            "test.js",
            r#"
                import * as assert from "assert";
                import { Foo, Bar, cross_item_construction } from "./out";

                export function test() {
                    const foo = new Foo(1);
                    assert.strictEqual(foo.get_number(), 1);
                    foo.free();

                    const foo2 = Foo.new(2);
                    assert.strictEqual(foo2.get_number(), 2);
                    foo2.free();

                    const bar = new Bar(3, 4);
                    assert.strictEqual(bar.get_sum(), 7);
                    bar.free();

                    const bar2 = Bar.other_name(5, 6);
                    assert.strictEqual(bar2.get_sum(), 11);
                    bar2.free();

                    assert.strictEqual(cross_item_construction().get_sum(), 15);
                }
            "#,
        )
        .test();
}

#[test]
fn empty_structs() {
    project()
        .debug(false)
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros, wasm_import_module)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            pub struct MissingClass {}

            #[wasm_bindgen]
            pub struct Other {}

            #[wasm_bindgen]
            impl Other { pub fn return_a_value() -> MissingClass { MissingClass {} } }
        "#,
        )
        .file(
            "test.js",
            r#"
                import { Other } from "./out";

                export function test() {
                    Other.return_a_value();
                }
            "#,
        )
        .test();
}

#[test]
fn public_fields() {
    project()
        .debug(false)
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros, wasm_import_module)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            #[derive(Default)]
            pub struct Foo {
                pub a: u32,
                pub b: f32,
                pub c: f64,
                pub d: i32,
            }

            #[wasm_bindgen]
            impl Foo {
                pub fn new() -> Foo {
                    Foo::default()
                }
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import { Foo } from "./out";
            import * as assert from "assert";

            export function test() {
                const a = Foo.new();
                assert.strictEqual(a.a, 0);
                a.a = 3;
                assert.strictEqual(a.a, 3);

                assert.strictEqual(a.b, 0);
                a.b = 7;
                assert.strictEqual(a.b, 7);

                assert.strictEqual(a.c, 0);
                a.c = 8;
                assert.strictEqual(a.c, 8);

                assert.strictEqual(a.d, 0);
                a.d = 3.3;
                assert.strictEqual(a.d, 3);
            }
        "#,
        )
        .test();
}

#[test]
fn using_self() {
    project()
        .debug(false)
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros, wasm_import_module)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            pub struct Foo {
            }

            #[wasm_bindgen]
            impl Foo {
                pub fn new() -> Self {
                    Foo {}
                }
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import { Foo } from "./out";

            export function test() {
                Foo.new().free();
            }
        "#,
        )
        .test();
}

#[test]
fn readonly_fields() {
    project()
        .debug(false)
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros, wasm_import_module)]

                extern crate wasm_bindgen;

                use wasm_bindgen::prelude::*;

                #[wasm_bindgen]
                #[derive(Default)]
                pub struct Foo {
                    #[wasm_bindgen(readonly)]
                    pub a: u32,
                }

                #[wasm_bindgen]
                impl Foo {
                    pub fn new() -> Foo {
                        Foo::default()
                    }
                }
            "#,
        )
        .file(
            "test.js",
            r#"
                import { Foo } from "./out";
                import * as assert from "assert";

                export function test() {
                    const a = Foo.new();
                    assert.strictEqual(a.a, 0);
                    assert.throws(() => a.a = 3, /has only a getter/);
                    a.free();
                }
            "#,
        )
        .test();
}

#[test]
fn double_consume() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros, wasm_import_module)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            pub struct Foo { }

            #[wasm_bindgen]
            impl Foo {
                #[wasm_bindgen(constructor)]
                pub fn new() -> Foo {
                    Foo {}
                }

                pub fn consume(self, other: Foo) {
                    drop(other);
                }
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import { Foo } from "./out";

            export function test() {
                const r = Foo.new();
                assert.throws(() => r.consume(r), /Attempt to use a moved value/);
            }
        "#)
        .test();
}
