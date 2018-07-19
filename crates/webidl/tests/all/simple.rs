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
                #![feature(use_extern_macros, wasm_import_module)]

                extern crate wasm_bindgen;

                use wasm_bindgen::prelude::*;

                pub mod foo;

                use foo::Foo;

                #[wasm_bindgen]
                pub fn test() {
                    let pi = Foo::new(3.14159).unwrap();
                    let e = Foo::new(2.71828).unwrap();
                    assert!(pi.my_cmp(&pi));
                    assert!(!pi.my_cmp(&e));
                    assert!(!e.my_cmp(&pi));
                    assert!(e.my_cmp(&e));
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
                #![feature(use_extern_macros, wasm_import_module)]

                extern crate wasm_bindgen;

                use wasm_bindgen::prelude::*;

                pub mod foo;

                use foo::Foo;

                #[wasm_bindgen]
                pub fn test() {
                    let x = Foo::new(3.14159).unwrap();
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
                #![feature(use_extern_macros, wasm_import_module)]

                extern crate wasm_bindgen;

                use wasm_bindgen::prelude::*;

                pub mod foo;

                use foo::Foo;

                #[wasm_bindgen]
                pub fn test() {
                    let x = Foo::new(3.14159).unwrap();
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
                #![feature(use_extern_macros, wasm_import_module)]

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
                #![feature(use_extern_macros, wasm_import_module)]

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

#[test]
fn one_method_using_an_undefined_import_doesnt_break_all_other_methods() {
    project()
        .file(
            "foo.webidl",
            r#"
                [Constructor()]
                interface Foo {
                    boolean ok_method();
                    boolean bad_method(UndefinedType undef);
                };
            "#,
        )
        .file(
            "foo.js",
            r#"
                export class Foo {
                    constructor() {}
                    ok_method() {
                        return true;
                    }
                }
            "#,
        )
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros, wasm_import_module)]
                extern crate wasm_bindgen;
                use wasm_bindgen::prelude::*;

                pub mod foo;

                #[wasm_bindgen]
                pub fn test() {
                    let f = foo::Foo::new().unwrap();
                    assert!(f.ok_method());
                }
            "#,
        )
        .test();
}

#[test]
fn unforgeable_is_structural() {
    project()
        .file(
            "foo.webidl",
            r#"
                [Constructor()]
                interface Foo {
                    [Unforgeable] readonly attribute short uno;
                                  readonly attribute short dos;
                };
            "#,
        )
        .file(
            "foo.js",
            r#"
                export class Foo {
                    constructor() {
                        this.uno = 1;
                    }
                    get dos() {
                        return 2;
                    }
                }
            "#,
        )
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros, wasm_import_module)]
                extern crate wasm_bindgen;
                use wasm_bindgen::prelude::*;

                pub mod foo;

                #[wasm_bindgen]
                pub fn test() {
                    let f = foo::Foo::new().unwrap();
                    assert_eq!(f.uno(), 1);
                    assert_eq!(f.dos(), 2);
                }
            "#,
        )
        .test();
}

#[test]
fn partial_interface() {
    project()
        .file(
            "foo.webidl",
            r#"
                [Constructor]
                interface Foo {
                    readonly attribute short un;
                    short deux();
                };

                partial interface Foo {
                    readonly attribute short trois;
                    short quatre();
                };
            "#,
        )
        .file(
            "foo.js",
            r#"
                export class Foo {
                    get un() {
                        return 1;
                    }
                    deux() {
                        return 2;
                    }
                    get trois() {
                        return 3;
                    }
                    quatre() {
                        return 4;
                    }
                }
            "#,
        )
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros, wasm_import_module)]
                extern crate wasm_bindgen;
                use wasm_bindgen::prelude::*;

                pub mod foo;

                #[wasm_bindgen]
                pub fn test() {
                    let f = foo::Foo::new().unwrap();
                    assert_eq!(f.un(), 1);
                    assert_eq!(f.deux(), 2);
                    assert_eq!(f.trois(), 3);
                    assert_eq!(f.quatre(), 4);
                }
            "#,
        )
        .test();
}

#[test]
fn mixin() {
    project()
        .file(
            "foo.webidl",
            r#"
                [Constructor(short bar)]
                interface Foo {
                    static attribute short defaultBar;
                };

                interface mixin Bar {
                    readonly attribute short bar;
                };

                partial interface mixin Bar {
                    void addToBar(short other);
                };

                Foo includes Bar;
            "#,
        )
        .file(
            "foo.js",
            r#"
                export class Foo {
                    constructor(bar) {
                        this._bar = bar | Foo.defaultBar;
                    }
                    static get defaultBar() {
                        return Foo._defaultBar;
                    }
                    static set defaultBar(defaultBar) {
                        Foo._defaultBar = defaultBar;
                    }
                    get bar() {
                        return this._bar;
                    }
                    addToBar(other) {
                        this._bar += other;
                    }
                }
            "#,
        )
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros, wasm_import_module)]
                extern crate wasm_bindgen;
                use wasm_bindgen::prelude::*;

                pub mod foo;

                use foo::Foo;

                #[wasm_bindgen]
                pub fn test() {
                    let f = Foo::new(1).unwrap();
                    assert_eq!(f.bar(), 1);
                    Foo::set_default_bar(7);
                    f.add_to_bar(Foo::default_bar());
                    assert_eq!(f.bar(), 8);
                }
            "#,
        )
        .test();
}
