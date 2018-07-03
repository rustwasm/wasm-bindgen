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
            "foo.js",
            r#"
                export class Foo {
                    constructor(value) {
                        this.value = value;
                    }
                    myCmp(other) {
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
                    // TODO: figure out why the following doesn't fail
                    // assert!(!pi.my_cmp(Foo::new(3.14159)));
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
            "foo.js",
            r#"
                export class Foo {
                    constructor(value) {
                        this._value = value;
                    }

                    get value() {
                        return this._value;
                    }

                    set value(value) {
                        this._value = value;
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
                    assert_eq!(x.value(), 3.14159);
                    assert_ne!(x.value(), 2.71828);
                    x.set_value(2.71828);
                    assert_ne!(x.value(), 3.14159);
                    assert_eq!(x.value(), 2.71828);
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
            "foo.js",
            r#"
                export class Foo {
                    constructor() {
                        this._value = 0;
                    }

                    get value(){
                        return this._value;
                    }
                }

                export class Bar extends Foo {
                    constructor(_value) {
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
                    assert_eq!(x.value(), 3.14159);
                    assert_ne!(x.value(), 0.);
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
                    static double swap(double value);
                };
            "#,
        )
        .file(
            "foo.js",
            r#"
                export class Foo {
                    static swap(value) {
                        const res = Foo.value;
                        Foo.value = value;
                        return res;
                    }
                }

                Foo.value = 0;
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
                    assert_eq!(Foo::swap(3.14159), 0.);
                    assert_eq!(Foo::swap(2.71828), 3.14159);
                    assert_ne!(Foo::swap(2.71828), 3.14159);
                    assert_eq!(Foo::swap(3.14159), 2.71828);
                    assert_ne!(Foo::swap(3.14159), 2.71828);
                }
            "#,
        )
        .test();
}

#[test]
fn static_property() {
    project()
        .file(
            "foo.webidl",
            r#"
                interface Foo {
                    static attribute double value;
                };
            "#,
        )
        .file(
            "foo.js",
            r#"
                export class Foo {
                    static get value(){
                        return Foo._value;
                    }

                    static set value(value) {
                        Foo._value = value;
                    }
                }

                Foo._value = 0;
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
                    assert_eq!(Foo::value(), 0.);
                    Foo::set_value(3.14159);
                    assert_eq!(Foo::value(), 3.14159);
                    assert_ne!(Foo::value(), 2.71828);
                    Foo::set_value(2.71828);
                    assert_eq!(Foo::value(), 2.71828);
                    assert_ne!(Foo::value(), 3.14159);
                }
            "#,
        )
        .test();
}
