use super::project;

#[test]
fn throws() {
    project()
        .file(
            "thang.webidl",
            r#"
                [Constructor(long value)]
                interface Thang {
                    [Throws]
                    attribute long ok_attr;
                    [Throws]
                    attribute long err_attr;

                    [Throws]
                    long ok_method();
                    [Throws]
                    long err_method();

                    [Throws]
                    static long ok_static_method();
                    [Throws]
                    static long err_static_method();

                    [Throws]
                    static attribute long ok_static_attr;
                    [Throws]
                    static attribute long err_static_attr;
                };
            "#,
        )
        .file(
            "thang.js",
            r#"
                export class Thang {
                    constructor(value) {
                        if (value % 2 == 0) {
                            throw new Error("only odd allowed");
                        }
                        this.value = value;
                    }

                    get ok_attr() { return this.value; }
                    set ok_attr(x) { }

                    get err_attr() { throw new Error("bad"); }
                    set err_attr(x) { throw new Error("bad"); }

                    ok_method() { return this.value + 1; }
                    err_method() { throw new Error("bad"); }

                    static ok_static_method() { return 1; }
                    static err_static_method() { throw new Error("bad"); }

                    static get ok_static_attr() { return 1; }
                    static set ok_static_attr(x) { }

                    static get err_static_attr() { throw new Error("bad"); }
                    static set err_static_attr(x) { throw new Error("bad"); }
                }
            "#,
        )
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros, wasm_import_module)]
                extern crate wasm_bindgen;
                use wasm_bindgen::prelude::*;

                pub mod thang;
                use thang::Thang;

                #[wasm_bindgen]
                pub fn test() {
                    assert!(Thang::new(0).is_err());
                    let thang = Thang::new(1).unwrap();

                    assert!(thang.ok_attr().is_ok());
                    assert!(thang.set_ok_attr(0).is_ok());

                    assert!(thang.err_attr().is_err());
                    assert!(thang.set_err_attr(0).is_err());

                    assert!(thang.ok_method().is_ok());
                    assert!(thang.err_method().is_err());

                    assert!(Thang::ok_static_method().is_ok());
                    assert!(Thang::err_static_method().is_err());

                    assert!(Thang::ok_static_attr().is_ok());
                    assert!(Thang::set_ok_static_attr(0).is_ok());

                    assert!(Thang::err_static_attr().is_err());
                    assert!(Thang::set_err_static_attr(0).is_err());
                }
            "#,
        )
        .test();
}
