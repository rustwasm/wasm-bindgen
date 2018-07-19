use super::project;

#[test]
fn bool() {
    project()
        .file(
            "foo.webidl",
            r#"
                interface Foo {
                    const boolean not_true = false;
                    const boolean not_false = true;
                };
            "#,
        )
        // a corresponding const in the js implementation is not required
        // value is taken directly from idl
        .file(
            "foo.js",
            r#"
                export class Foo {
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
                    let falsish: bool = Foo::NOT_TRUE;
                    assert!(!falsish);
                    let trueish: bool = Foo::NOT_FALSE;
                    assert!(trueish);
                }
            "#,
        )
        .test();
}

#[test]
fn ints() {
    project()
        .file(
            "foo.webidl",
            r#"
                interface Byte {
                    const byte imin = -128;
                    const byte imax = 127;
                    const octet umin = 0;
                    const octet umax = 255;
                };
                interface Short {
                    const short imin = -32768;
                    const short imax = 32767;
                    const unsigned short umin = 0;
                    const unsigned short umax = 65535;
                };
                interface Long {
                    const long imin = -2147483648;
                    const long imax = 2147483647;
                    const unsigned long umin = 0;
                    const unsigned long umax = 4294967295;
                };
                interface LongLong {
                    const long long imin = -9223372036854775808;
                    const long long imax = 9223372036854775807;
                    const unsigned long long umin = 0;
                    const unsigned long long umax = 18446744073709551615;
                };
            "#,
        )
        // a corresponding const in the js implementation is not required
        // value is taken directly from idl
        .file(
            "foo.js",
            r#"
                export class Byte {
                }
                export class Short {
                }
                export class Long {
                }
                export class LongLong {
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
                    assert_eq!(foo::Byte::IMIN, i8::min_value());
                    assert_eq!(foo::Byte::IMAX, i8::max_value());
                    assert_eq!(foo::Byte::UMIN, u8::min_value());
                    assert_eq!(foo::Byte::UMAX, u8::max_value());

                    assert_eq!(foo::Short::IMIN, i16::min_value());
                    assert_eq!(foo::Short::IMAX, i16::max_value());
                    assert_eq!(foo::Short::UMIN, u16::min_value());
                    assert_eq!(foo::Short::UMAX, u16::max_value());

                    assert_eq!(foo::Long::IMIN, i32::min_value());
                    assert_eq!(foo::Long::IMAX, i32::max_value());
                    assert_eq!(foo::Long::UMIN, u32::min_value());
                    assert_eq!(foo::Long::UMAX, u32::max_value());

                    assert_eq!(foo::LongLong::IMIN, i64::min_value());
                    assert_eq!(foo::LongLong::IMAX, i64::max_value());
                    assert_eq!(foo::LongLong::UMIN, u64::min_value());
                    assert_eq!(foo::LongLong::UMAX, u64::max_value());
                }
            "#,
        )
        .test();
}

#[test]
fn floats() {
    project()
        .file(
            "foo.webidl",
            r#"
                interface floats {
                    const float f = 0.0;
                    const unrestricted float neg_inf = -Infinity;
                    const unrestricted float inf = Infinity;
                    const unrestricted float nan = NaN;
                };
                interface doubles {
                    const double d = 0.0;
                    const unrestricted double neg_inf = -Infinity;
                    const unrestricted double inf = Infinity;
                    const unrestricted double nan = NaN;
                };
            "#,
        )
        // a corresponding const in the js implementation is not required
        // value is taken directly from idl
        .file(
            "foo.js",
            r#"
                export class floats {
                }
                export class doubles {
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
                    assert_eq!(foo::Floats::F, 0.0_f32);
                    assert!(foo::Floats::NEG_INF.is_infinite());
                    assert!(foo::Floats::NEG_INF.is_sign_negative());
                    assert!(foo::Floats::INF.is_infinite());
                    assert!(foo::Floats::INF.is_sign_positive());
                    assert!(foo::Floats::NAN.is_nan());

                    assert_eq!(foo::Doubles::D, 0.0_f64);
                    assert!(foo::Doubles::NEG_INF.is_infinite());
                    assert!(foo::Doubles::NEG_INF.is_sign_negative());
                    assert!(foo::Doubles::INF.is_infinite());
                    assert!(foo::Doubles::INF.is_sign_positive());
                    assert!(foo::Doubles::NAN.is_nan());
                }
            "#,
        )
        .test();
}
