#![allow(non_snake_case)]

use super::project;

#[test]
fn get_date() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::Date;

            #[wasm_bindgen]
            pub fn get_date(this: &Date) -> u32 {
                this.get_date()
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let date = new Date(1993, 6, 28, 14, 39, 7);

                assert.equal(wasm.get_date(date), 28);
            }
        "#,
        )
        .test()
}

#[test]
fn get_day() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::Date;

            #[wasm_bindgen]
            pub fn get_day(this: &Date) -> u32 {
                this.get_day()
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let date = new Date('August 19, 1975 23:15:30');

                assert.equal(wasm.get_day(date), 2);
            }
        "#,
        )
        .test()
}

#[test]
fn get_full_year() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::Date;

            #[wasm_bindgen]
            pub fn get_full_year(this: &Date) -> u32 {
                this.get_full_year()
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let date = new Date('July 20, 1969 00:20:18');
                let abbrDate = new Date('Thu, 06 Sep 12 00:00:00');

                assert.equal(wasm.get_full_year(date), 1969);
                assert.equal(wasm.get_full_year(abbrDate), 2012);
            }
        "#,
        )
        .test()
}

#[test]
fn get_hours() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::Date;

            #[wasm_bindgen]
            pub fn get_hours(this: &Date) -> u32 {
                this.get_hours()
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let date = new Date('March 13, 08 04:20');

                assert.equal(wasm.get_hours(date), 4);
            }
        "#,
        )
        .test()
}

#[test]
fn get_milliseconds() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::Date;

            #[wasm_bindgen]
            pub fn get_milliseconds(this: &Date) -> u32 {
                this.get_milliseconds()
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let date = new Date('July 20, 69 00:20:18');
                let dateWithMs = new Date('July 20, 69 00:20:18:123');

                assert.equal(wasm.get_milliseconds(date), 0);
                assert.equal(wasm.get_milliseconds(dateWithMs), 123);
            }
        "#,
        )
        .test()
}

#[test]
fn get_minutes() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::Date;

            #[wasm_bindgen]
            pub fn get_minutes(this: &Date) -> u32 {
                this.get_minutes()
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let date = new Date('March 13, 08 04:20');

                assert.equal(wasm.get_minutes(date), 20);
            }
        "#,
        )
        .test()
}

#[test]
fn get_month() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::Date;

            #[wasm_bindgen]
            pub fn get_month(this: &Date) -> u32 {
                this.get_month()
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let date = new Date('July 20, 69 00:20:18');

                assert.equal(wasm.get_month(date), 6);
            }
        "#,
        )
        .test()
}

#[test]
fn get_seconds() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::Date;

            #[wasm_bindgen]
            pub fn get_seconds(this: &Date) -> u32 {
                this.get_seconds()
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let date = new Date('July 20, 69 00:20:18');

                assert.equal(wasm.get_seconds(date), 18);
            }
        "#,
        )
        .test()
}

#[test]
fn new() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::Date;

            #[wasm_bindgen]
            pub fn new_date() -> Date {
                Date::new()
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(typeof wasm.new_date(), "object");
            }
        "#,
        )
        .test()
}

#[test]
fn now() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::Date;

            #[wasm_bindgen]
            pub fn now() -> f64 {
                Date::now()
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(typeof wasm.now(), "number");
            }
        "#)
        .test()
}

#[test]
fn to_date_string() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::{Date, JsString};

            #[wasm_bindgen]
            pub fn to_date_string(this: &Date) -> JsString {
                this.to_date_string()
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let date = new Date(1993, 6, 28, 14, 39, 7);

                assert.equal(wasm.to_date_string(date), 'Wed Jul 28 1993');
            }
        "#,
        )
        .test()
}

#[test]
fn to_iso_string() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::{Date, JsString};

            #[wasm_bindgen]
            pub fn to_iso_string(this: &Date) -> JsString {
                this.to_iso_string()
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let date = new Date('05 October 2011 14:48 UTC');

                assert.equal(wasm.to_iso_string(date), '2011-10-05T14:48:00.000Z');
            }
        "#,
        )
        .test()
}

#[test]
fn to_json() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::{Date, JsString};

            #[wasm_bindgen]
            pub fn to_json(this: &Date) -> JsString {
                this.to_json()
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let date = new Date('August 19, 1975 23:15:30 UTC');

                assert.equal(wasm.to_json(date), '1975-08-19T23:15:30.000Z');
            }
        "#,
        )
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
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let date = new Date(Date.UTC(2012, 11, 20, 3, 0, 0));
                let options = { weekday: 'long', year: 'numeric', month: 'long', day: 'numeric' };

                let output = wasm.to_locale_date_string(date, 'de-DE', options)
                assert.equal(typeof output, 'string');
                assert.ok(output.length > 0);
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
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let date = new Date(Date.UTC(2012, 11, 20, 3, 0, 0));
                let output = wasm.to_locale_string(date, 'en-GB', { timeZone: 'UTC' });
                assert.equal(typeof output, 'string');
                assert.ok(output.length > 0);
            }
        "#)
        .test()
}

#[test]
fn to_locale_time_string() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::{Date, JsString};

            #[wasm_bindgen]
            pub fn to_locale_time_string(this: &Date, locale: JsString) -> JsString {
                this.to_locale_time_string(locale)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let date = new Date('August 19, 1975 23:15:30');
                assert.equal(wasm.to_locale_time_string(date, 'en-US'), "11:15:30 PM");
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
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::{Date, JsString};

            #[wasm_bindgen]
            pub fn to_string(this: &Date) -> JsString {
                this.to_string()
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let date = new Date('August 19, 1975 23:15:30');
                assert.equal(wasm.to_string(date).substring(0, 15), "Tue Aug 19 1975");
            }
        "#,
        )
        .test()
}

#[test]
fn to_time_string() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::{Date, JsString};

            #[wasm_bindgen]
            pub fn to_time_string(this: &Date) -> JsString {
                this.to_time_string()
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let date = new Date('August 19, 1975 23:15:30');
                assert.equal(wasm.to_time_string(date).substring(0, 8), "23:15:30");
            }
        "#,
        )
        .test()
}

#[test]
fn to_utc_string() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::{Date, JsString};

            #[wasm_bindgen]
            pub fn to_utc_string(this: &Date) -> JsString {
                this.to_utc_string()
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let date = new Date('14 Jun 2017 00:00:00 PDT');
                assert.equal(wasm.to_utc_string(date), "Wed, 14 Jun 2017 07:00:00 GMT");
            }
        "#,
        )
        .test()
}

#[test]
fn utc() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::Date;

            #[wasm_bindgen]
            pub fn utc() -> f64 {
                Date::utc(2018f64, 6f64)
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.utc(), 1530403200000);
            }
        "#)
        .test()
}

#[test]
fn value_of() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::Date;

            #[wasm_bindgen]
            pub fn js_value_of(this: &Date) -> Date {
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
                let date = new Date(Date.UTC(96, 1, 2, 3, 4, 5));
                assert.equal(wasm.js_value_of(date), 823230245000);
            }
        "#,
        )
        .test()
}
