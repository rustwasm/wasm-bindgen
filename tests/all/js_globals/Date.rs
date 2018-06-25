#![allow(non_snake_case)]

use super::project;

#[test]
fn to_string() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::{Date, JsString};

            #[wasm_bindgen]
            pub fn to_string(this: &Date) -> JsString {
                this.to_string()
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let date = new Date('August 19, 1975 23:15:30');
                assert.equal(wasm.to_string(date).substring(0, 15), "Tue Aug 19 1975");
            }
        "#)
        .test()
}

#[test]
fn to_time_string() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::{Date, JsString};

            #[wasm_bindgen]
            pub fn to_time_string(this: &Date) -> JsString {
                this.to_time_string()
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let date = new Date('August 19, 1975 23:15:30');
                assert.equal(wasm.to_time_string(date).substring(0, 8), "23:15:30");
            }
        "#)
        .test()
}

#[test]
fn to_utc_string() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::{Date, JsString};

            #[wasm_bindgen]
            pub fn to_utc_string(this: &Date) -> JsString {
                this.to_utc_string()
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let date = new Date('14 Jun 2017 00:00:00 PDT');
                assert.equal(wasm.to_utc_string(date), "Wed, 14 Jun 2017 07:00:00 GMT");
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
            use wasm_bindgen::js::Date;

            #[wasm_bindgen]
            pub fn js_value_of(this: &Date) -> Date {
                this.value_of()
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let date = new Date(Date.UTC(96, 1, 2, 3, 4, 5));
                assert.equal(wasm.js_value_of(date), 823230245000);
            }
        "#)
        .test()
}
