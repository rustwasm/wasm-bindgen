#![allow(non_snake_case)]

use super::project;

#[test]
fn to_json() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::{Date, JsString};

            #[wasm_bindgen]
            pub fn to_json(this: &Date) -> JsString {
                this.to_json()
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let date = new Date('August 19, 1975 23:15:30 UTC');

                assert.equal(wasm.to_json(date), '1975-08-19T23:15:30.000Z');
            }
        "#)
        .test()
}

#[test]
fn to_locale_date_string() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use JsValue;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::{Date, JsString};

            #[wasm_bindgen]
            pub fn to_locale_date_string(this: &Date, locale: JsString, options: JsValue) -> JsString {
                this.to_locale_date_string(locale, options)
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let date = new Date(Date.UTC(2012, 11, 20, 3, 0, 0));
                let options = { weekday: 'long', year: 'numeric', month: 'long', day: 'numeric' };

                assert.equal(wasm.to_locale_date_string(date, 'de-DE', options), '2012 M12 20, Thu');
            }
        "#)
        .test()
}

#[test]
fn to_locale_string() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use JsValue;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::{Date, JsString};

            #[wasm_bindgen]
            pub fn to_locale_string(this: &Date, locale: JsString, options: JsValue) -> JsString {
                this.to_locale_string(locale, options)
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let date = new Date(Date.UTC(2012, 11, 20, 3, 0, 0));
                assert.equal(wasm.to_locale_string(date, 'en-GB', { timeZone: 'UTC' }), "12/20/2012, 3:00:00 AM");
            }
        "#)
        .test()
}

#[test]
fn to_locale_time_string() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::{Date, JsString};

            #[wasm_bindgen]
            pub fn to_locale_time_string(this: &Date, locale: JsString) -> JsString {
                this.to_locale_time_string(locale)
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let date = new Date('August 19, 1975 23:15:30');
                assert.equal(wasm.to_locale_time_string(date, 'en-US'), "11:15:30 PM");
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
