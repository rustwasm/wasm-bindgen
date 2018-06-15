use super::project;

#[test]
fn method() {
    project()
        .file(
            "foo.webidl",
            r#"
            [Constructor(double value)]
            interface Foo {
                [Pure]
                boolean myCmp(Foo bar);
            };
        "#,
        )
        .file(
            "foo.ts",
            r#"
            export class Foo {
                constructor(private value: number) {
                    this.value = value;
                }
                myCmp(other: Foo): boolean {
                    return this.value === other.value;
                }
            }
            "#,
        )
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section, wasm_import_module)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            pub mod foo;

            use foo::Foo;

            #[wasm_bindgen]
            pub fn test() {
                let pi = Foo::new(3.14159);
                let e = Foo::new(2.71828);
                let tmp = pi.my_cmp(Foo::new(3.14159));
                assert!(tmp);
                let tmp =!pi.my_cmp(Foo::new(2.71828));
                assert!(tmp);
                let tmp = !e.my_cmp(Foo::new(3.14159));
                assert!(tmp);
                let tmp = e.my_cmp(Foo::new(2.71828));
                assert!(tmp);
            }
        "#,
        )
        .test();
}

#[test]
fn property() {
    project()
        .file(
            "foo.webidl",
            r#"
            [Constructor(double value)]
            interface Foo {
                [Pure]
                attribute double value;
            };
        "#,
        )
        .file(
            "foo.ts",
            r#"
            export class Foo {
                constructor(private _value: number) {
                    this._value = _value;
                }

                get value(): number {
                    return this._value;
                }

                set value(_value: number) {
                    this._value = _value;
                }
            }
            "#,
        )
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section, wasm_import_module)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            pub mod foo;

            use foo::Foo;

            #[wasm_bindgen]
            pub fn test() {
                let x = Foo::new(3.14159);
                let tmp = x.value() == 3.14159;
                assert!(tmp);
                let tmp = x.value() != 2.71828;
                assert!(tmp);
                x.set_value(2.71828);
                let tmp = x.value() != 3.14159;
                assert!(tmp);
                let tmp = x.value() == 2.71828;
                assert!(tmp);
            }
        "#,
        )
        .test();
}

#[test]
fn named_constructor() {
    project()
        .file(
            "foo.webidl",
            r#"
            [NamedConstructor=Bar(double value)]
            interface Foo {
                [Pure]
                readonly attribute double value;
            };
        "#,
        )
        .file(
            // Not a perfect test, but it gets the job done.
            "foo.ts",
            r#"
            export class Foo {
                protected _value: number = 0;
                get value(): number {
                    return this._value;
                }
            }

            export class Bar extends Foo {
                constructor(_value: number) {
                    super();
                    this._value = _value;
                }
            }
            "#,
        )
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section, wasm_import_module)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            pub mod foo;

            use foo::Foo;

            #[wasm_bindgen]
            pub fn test() {
                let x = Foo::new(3.14159);
                let tmp = x.value() == 3.14159;
                assert!(tmp);
                let tmp = x.value() != 0.;
                assert!(tmp);
            }
        "#,
        )
        .test();
}

#[test]
fn static_method() {
    project()
        .file(
            "foo.webidl",
            r#"
            interface Foo {
                [Pure]
                static double swap(double value);
            };
        "#,
        )
        .file(
            "foo.ts",
            r#"
            export class Foo {
                private static value: number = 0;
                static swap(value: number): number {
                    const res = Foo.value;
                    Foo.value = value;
                    return res;
                }
            }
            "#,
        )
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section, wasm_import_module)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            pub mod foo;

            use foo::Foo;

            #[wasm_bindgen]
            pub fn test() {
                let tmp = Foo::swap(3.14159) == 0.;
                assert!(tmp);
                let tmp = Foo::swap(2.71828) == 3.14159;
                assert!(tmp);
                let tmp = Foo::swap(2.71828) != 3.14159;
                assert!(tmp);
                let tmp = Foo::swap(3.14159) == 2.71828;
                assert!(tmp);
                let tmp = Foo::swap(3.14159) != 2.71828;
                assert!(tmp);
            }
        "#,
        )
        .test();
}
