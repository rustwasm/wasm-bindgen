#![allow(non_snake_case)]

use super::project;

#[test]
fn is_finite() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn is_finite(value: &JsValue) -> bool {
                js::Number::is_finite(value)
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.is_finite(42), true);
                assert.equal(wasm.is_finite(42.1), true);
                assert.equal(wasm.is_finite('42'), false);
                assert.equal(wasm.is_finite(NaN), false);
                assert.equal(wasm.is_finite(Infinity), false);
            }
        "#)
        .test()
}

#[test]
fn is_integer() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn is_integer(value: &JsValue) -> bool {
                js::Number::is_integer(value)
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.ok(wasm.is_integer(123));
                assert.ok(!wasm.is_integer(123.45));
            }
        "#)
        .test()
}

#[test]
fn is_safe_integer() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn is_safe_integer(value: &JsValue) -> bool {
                js::Number::is_safe_integer(value)
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.is_safe_integer(42), true);
                assert.equal(wasm.is_safe_integer(Math.pow(2, 53) - 1), true);
                assert.equal(wasm.is_safe_integer(Math.pow(2, 53)), false);
                assert.equal(wasm.is_safe_integer('42'), false);
                assert.equal(wasm.is_safe_integer(42.1), false);
                assert.equal(wasm.is_safe_integer(NaN), false);
                assert.equal(wasm.is_safe_integer(Infinity), false);
            }
        "#)
        .test()
}

#[test]
fn new() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::Number;

            #[wasm_bindgen]
            pub fn new_number() -> Number {
                Number::new(JsValue::from(42))
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(typeof wasm.new_number(), "object");
                assert.equal(wasm.new_number(), 42);
            }
        "#)
        .test()
}

#[test]
fn parse_int_float() {
    project()
        .file(
            "src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::Number;

            #[wasm_bindgen]
            pub fn parse_int(text: &str, radix: u8) -> Number {
                Number::parse_int(text, radix)
            }

            #[wasm_bindgen]
            pub fn parse_float(text: &str) -> Number {
                Number::parse_float(text)
            }
        "#,
        )
        .file(
            "test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.parse_int("42", 10), 42);
                assert.equal(wasm.parse_int("42", 16), 66); // 0x42 == 66
                assert.ok(Number.isNaN(wasm.parse_int("invalid int", 10)), "should be NaN");

                assert.equal(wasm.parse_float("123456.789"), 123456.789);
                assert.ok(Number.isNaN(wasm.parse_float("invalid float")), "should be NaN");
            }
        "#,
        )
        .test()
}

#[test]
fn to_locale_string() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn to_locale_string(this: &js::Number, locale: &str) -> js::JsString {
                this.to_locale_string(locale)
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let number = 1234.45;
                assert.equal(wasm.to_locale_string(number, "en-US"), "1,234.45");
                // TODO: these tests seems to be system dependent, disable for now
                // assert.equal(wasm.to_locale_string(number, "de-DE"), "1,234.45");
                // assert.equal(wasm.to_locale_string(number, "zh-Hans-CN-u-nu-hanidec"), "1,234.45");
            }
        "#)
        .test()
}

#[test]
fn to_precision() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn to_precision(this: &js::Number, precision: u8) -> js::JsString {
                let result = this.to_precision(precision);
                let result = match result {
                    Ok(num) => num,
                    Err(_err) => "RangeError".into()
                };
                result
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.to_precision(0.1, 3), "0.100");
                assert.equal(wasm.to_precision(10, 101), "RangeError");
            }
        "#,
        )
        .test()
}

#[test]
fn to_string() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn to_string(this: &js::Number, radix: u8) -> js::JsString {
                let result = this.to_string(radix);
                let result = match result {
                    Ok(num) => num,
                    Err(_err) => "RangeError".into()
                };
                result
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let number = 42;
                assert.equal(wasm.to_string(number, 10), "42");
                assert.equal(wasm.to_string(233, 16), "e9");
                assert.equal(wasm.to_string(number, 100), "RangeError");
            }
        "#,
        )
        .test()
}

#[test]
fn value_of() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn js_value_of(this: &js::Number) -> f64 {
                this.value_of()
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let number = 42;
                assert.equal(wasm.js_value_of(number), 42);
                assert.equal(typeof wasm.js_value_of(number), "number");
            }
        "#,
        )
        .test()
}

#[test]
fn to_fixed() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn to_fixed(this: &js::Number, digits: u8) -> js::JsString {
                let result = this.to_fixed(digits);
                let result = match result {
                    Ok(num) => num,
                    Err(_err) => "RangeError".into()
                };
                result
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.to_fixed(123.456, 2), "123.46");
                assert.equal(wasm.to_fixed(10, 101), "RangeError");
            }
        "#,
        )
        .test()
}

#[test]
fn to_exponential() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn to_exponential(this: &js::Number, fraction_digits: u8) -> js::JsString {
                let result = this.to_exponential(fraction_digits);
                let result = match result {
                    Ok(num) => num,
                    Err(_err) => "RangeError".into()
                };
                result
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.to_exponential(123456, 2), "1.23e+5");
                assert.equal(wasm.to_exponential(10, 101), "RangeError");
            }
        "#,
        )
        .test()
}
