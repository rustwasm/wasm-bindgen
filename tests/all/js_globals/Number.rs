#![allow(non_snake_case)]

use super::project;


#[test]
fn to_locale_string() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn to_locale_string(this: &js::Number, locale: String) -> String {
                this.to_locale_string(locale)
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let number = 1234.45;
                assert.equal(wasm.to_locale_string(number, "de-DE"), "1,234.45");
                assert.equal(wasm.to_locale_string(number, "en-US"), "1,234.45");
                assert.equal(wasm.to_locale_string(number, "zh-Hans-CN-u-nu-hanidec"), "1,234.45");
            }
        "#)
        .test()
}

#[test]
fn to_precision() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn to_precision(this: &js::Number, precision: u8) -> String {
                let result = this.to_precision(precision);
                let result = match result {
                    Ok(num) => num,
                    Err(_err) => "RangeError".to_string()
                };
                result
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.to_precision(0.1, 3), "0.100");
                assert.equal(wasm.to_precision(10, 101), "RangeError");
            }
        "#)
        .test()
}

#[test]
fn to_string() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn to_string(this: &js::Number, radix: u8) -> String {
                let result = this.to_string(radix);
                let result = match result {
                    Ok(num) => num,
                    Err(_err) => "RangeError".to_string()
                };
                result
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let number = 42;
                assert.equal(wasm.to_string(number, 10), "42");
                assert.equal(wasm.to_string(233, 16), "e9");
                assert.equal(wasm.to_string(number, 100), "RangeError");
            }
        "#)
        .test()
}

#[test]
fn value_of() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn js_value_of(this: &js::Number) -> js::Number {
                this.value_of()
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let number = 42;
                assert.equal(wasm.js_value_of(number), 42);
                assert.equal(typeof wasm.js_value_of(number), "number");
            }
        "#)
        .test()
}
