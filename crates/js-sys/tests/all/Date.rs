#![allow(non_snake_case)]

use super::project;

#[test]
fn get_date() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

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
            #![feature(use_extern_macros)]

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
            #![feature(use_extern_macros)]

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
            #![feature(use_extern_macros)]

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
            #![feature(use_extern_macros)]

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
            #![feature(use_extern_macros)]

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
            #![feature(use_extern_macros)]

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
            #![feature(use_extern_macros)]

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
fn get_time() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::Date;

            #[wasm_bindgen]
            pub fn get_time(this: &Date) -> f64 {
                this.get_time()
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let date = new Date('July 20, 69 00:20:18 GMT+00:00');

                assert.equal(wasm.get_time(date), -14254782000);
            }
        "#,
        )
        .test()
}

#[test]
fn get_timezone_offset() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::Date;

            #[wasm_bindgen]
            pub fn get_timezone_offset(this: &Date) -> f64 {
                this.get_timezone_offset()
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let date1 = new Date('August 19, 1975 23:15:30 GMT+07:00');
                let date2 = new Date('August 19, 1975 23:15:30 GMT-02:00');

                assert.equal(typeof wasm.get_timezone_offset(date1), "number");
                assert.equal(wasm.get_timezone_offset(date1), wasm.get_timezone_offset(date2));
            }
        "#,
        )
        .test()
}

#[test]
fn get_utc_date() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::Date;

            #[wasm_bindgen]
            pub fn get_utc_date(this: &Date) -> u32 {
                this.get_utc_date()
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let date1 = new Date('August 19, 1975 23:15:30 GMT+11:00');
                let date2 = new Date('August 19, 1975 23:15:30 GMT-11:00');

                assert.equal(wasm.get_utc_date(date1), 19);
                assert.equal(wasm.get_utc_date(date2), 20);
            }
        "#,
        )
        .test()
}

#[test]
fn get_utc_day() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::Date;

            #[wasm_bindgen]
            pub fn get_utc_day(this: &Date) -> u32 {
                this.get_utc_day()
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let date1 = new Date('August 19, 1975 23:15:30 GMT+11:00');
                let date2 = new Date('August 19, 1975 23:15:30 GMT-11:00');

                assert.equal(wasm.get_utc_day(date1), 2);
                assert.equal(wasm.get_utc_day(date2), 3);
            }
        "#,
        )
        .test()
}

#[test]
fn get_utc_full_year() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::Date;

            #[wasm_bindgen]
            pub fn get_utc_full_year(this: &Date) -> u32 {
                this.get_utc_full_year()
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let date1 = new Date('December 31, 1975, 23:15:30 GMT+11:00');
                let date2 = new Date('December 31, 1975, 23:15:30 GMT-11:00');

                assert.equal(wasm.get_utc_full_year(date1), 1975);
                assert.equal(wasm.get_utc_full_year(date2), 1976);
            }
        "#,
        )
        .test()
}

#[test]
fn get_utc_hours() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::Date;

            #[wasm_bindgen]
            pub fn get_utc_hours(this: &Date) -> u32 {
                this.get_utc_hours()
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let date1 = new Date('December 31, 1975, 23:15:30 GMT+11:00');
                let date2 = new Date('December 31, 1975, 23:15:30 GMT-11:00');

                assert.equal(wasm.get_utc_hours(date1), 12);
                assert.equal(wasm.get_utc_hours(date2), 10);
            }
        "#,
        )
        .test()
}

#[test]
fn get_utc_milliseconds() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::Date;

            #[wasm_bindgen]
            pub fn get_utc_milliseconds(this: &Date) -> u32 {
                this.get_utc_milliseconds()
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let date = new Date('2018-01-02T03:04:05.678Z');

                assert.equal(wasm.get_utc_milliseconds(date), 678);
            }
        "#,
        )
        .test()
}

#[test]
fn get_utc_minutes() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::Date;

            #[wasm_bindgen]
            pub fn get_utc_minutes(this: &Date) -> u32 {
                this.get_utc_minutes()
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let date1 = new Date('1 January 2000 03:15:30 GMT+07:00');
                let date2 = new Date('1 January 2000 03:15:30 GMT+03:30');

                assert.equal(wasm.get_utc_minutes(date1), 15);
                assert.equal(wasm.get_utc_minutes(date2), 45);
            }
        "#,
        )
        .test()
}

#[test]
fn get_utc_month() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::Date;

            #[wasm_bindgen]
            pub fn get_utc_month(this: &Date) -> u32 {
                this.get_utc_month()
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let date1 = new Date('December 31, 1975, 23:15:30 GMT+11:00');
                let date2 = new Date('December 31, 1975, 23:15:30 GMT-11:00');

                assert.equal(wasm.get_utc_month(date1), 11);
                assert.equal(wasm.get_utc_month(date2), 0);
            }
        "#,
        )
        .test()
}

#[test]
fn get_utc_seconds() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::Date;

            #[wasm_bindgen]
            pub fn get_utc_seconds(this: &Date) -> u32 {
                this.get_utc_seconds()
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let date = new Date('July 20, 1969, 20:18:04 UTC');

                assert.equal(wasm.get_utc_seconds(date), 4);
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
            #![feature(use_extern_macros)]

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
            #![feature(use_extern_macros)]

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
fn parse() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::{Date, JsString};

            #[wasm_bindgen]
            pub fn parse(date: JsString) -> f64 {
                Date::parse(date)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let date = wasm.parse('04 Dec 1995 00:12:00 GMT');
                let unixTimeZero = wasm.parse('01 Jan 1970 00:00:00 GMT');

                assert.equal(date, 818035920000);
                assert.equal(unixTimeZero, 0);
            }
        "#,
        )
        .test()
}

#[test]
fn set_date() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::Date;

            #[wasm_bindgen]
            pub fn set_date(this: &Date, day: u32) -> f64 {
                this.set_date(day)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let event1 = new Date('August 19, 1975 23:15:30');
                let event2 = new Date('August 24, 1975 23:15:30');

                let eventMsFromUnixEpoch = wasm.set_date(event1, 24);

                assert.equal(eventMsFromUnixEpoch, event2.getTime());
                assert.equal(event1.getTime(), event2.valueOf());
                assert.equal(event1.getDate(), 24);
            }
        "#,
        )
        .test()
}

#[test]
fn set_full_year() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::Date;

            #[wasm_bindgen]
            pub fn set_full_year(this: &Date, year: u32) -> f64 {
                this.set_full_year(year)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let event1 = new Date('August 19, 1975 23:15:30');
                let event2 = new Date('August 19, 1976 23:15:30');

                let eventMsFromUnixEpoch = wasm.set_full_year(event1, 1976);

                assert.equal(eventMsFromUnixEpoch, event2.getTime());
                assert.equal(event1.getTime(), event2.valueOf());
                assert.equal(event1.getFullYear(), 1976);
            }
        "#,
        )
        .test()
}

#[test]
fn set_hours() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::Date;

            #[wasm_bindgen]
            pub fn set_hours(this: &Date, hours: u32) -> f64 {
                this.set_hours(hours)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let event1 = new Date('August 19, 1975 23:15:30');
                let event2 = new Date('August 19, 1975 20:15:30');

                let eventMsFromUnixEpoch = wasm.set_hours(event1, 20);

                assert.equal(eventMsFromUnixEpoch, event2.getTime());
                assert.equal(event1.getTime(), event2.valueOf());
                assert.equal(event1.getHours(), 20);
            }
        "#,
        )
        .test()
}

#[test]
fn set_milliseconds() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::Date;

            #[wasm_bindgen]
            pub fn set_milliseconds(this: &Date, milliseconds: u32) -> f64 {
                this.set_milliseconds(milliseconds)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let event = new Date('August 19, 1975 23:15:30');

                let eventMsFromUnixEpoch = wasm.set_milliseconds(event, 456);

                assert.equal(eventMsFromUnixEpoch, event.getTime());
                assert.equal(event.getMilliseconds(), 456);
            }
        "#,
        )
        .test()
}

#[test]
fn set_minutes() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::Date;

            #[wasm_bindgen]
            pub fn set_minutes(this: &Date, minutes: u32) -> f64 {
                this.set_minutes(minutes)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let event1 = new Date('August 19, 1975 23:15:30');
                let event2 = new Date('August 19, 1975 23:45:30');

                let eventMsFromUnixEpoch = wasm.set_minutes(event1, 45);

                assert.equal(eventMsFromUnixEpoch, event2.getTime());
                assert.equal(event1.getTime(), event2.valueOf());
                assert.equal(event1.getMinutes(), 45);
            }
        "#,
        )
        .test()
}

#[test]
fn set_month() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::Date;

            #[wasm_bindgen]
            pub fn set_month(this: &Date, month: u32) -> f64 {
                this.set_month(month)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let event1 = new Date('August 19, 1975 23:15:30');
                let event2 = new Date('April 19, 1975 23:15:30');

                let eventMsFromUnixEpoch = wasm.set_month(event1, 3);

                assert.equal(eventMsFromUnixEpoch, event2.getTime());
                assert.equal(event1.getTime(), event2.valueOf());
                assert.equal(event1.getMonth(), 3);
            }
        "#,
        )
        .test()
}

#[test]
fn set_seconds() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::Date;

            #[wasm_bindgen]
            pub fn set_seconds(this: &Date, seconds: u32) -> f64 {
                this.set_seconds(seconds)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let event1 = new Date('August 19, 1975 23:15:30');
                let event2 = new Date('August 19, 1975 23:15:42');

                let eventMsFromUnixEpoch = wasm.set_seconds(event1, 42);

                assert.equal(eventMsFromUnixEpoch, event2.getTime());
                assert.equal(event1.getTime(), event2.valueOf());
                assert.equal(event1.getSeconds(), 42);
            }
        "#,
        )
        .test()
}

#[test]
fn set_time() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::Date;

            #[wasm_bindgen]
            pub fn set_time(this: &Date, time: f64) -> f64 {
                this.set_time(time)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let event1 = new Date('July 1, 1999');
                let event2 = new Date();

                let eventMsFromUnixEpoch = wasm.set_time(event2, event1.getTime());

                assert.equal(eventMsFromUnixEpoch, event2.getTime());
                assert.equal(event1.valueOf(), event2.getTime());
            }
        "#,
        )
        .test()
}

#[test]
fn set_utc_date() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::Date;

            #[wasm_bindgen]
            pub fn set_utc_date(this: &Date, day: u32) -> f64 {
                this.set_utc_date(day)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let event1 = new Date('August 19, 1975 23:15:30 GMT-3:00');
                let event2 = new Date('August 19, 1975 02:15:30 GMT');

                let eventMsFromUnixEpoch = wasm.set_utc_date(event1, 19);

                assert.equal(eventMsFromUnixEpoch, event2.getTime());
                assert.equal(event1.getTime(), event2.valueOf());
                assert.equal(event1.getUTCDate(), 19);
            }
        "#,
        )
        .test()
}

#[test]
fn set_utc_full_year() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::Date;

            #[wasm_bindgen]
            pub fn set_utc_full_year(this: &Date, year: u32) -> f64 {
                this.set_utc_full_year(year)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let event1 = new Date('December 31, 1975 23:15:30 GMT-3:00');
                let event2 = new Date('January 01, 1975 02:15:30 GMT');

                let eventMsFromUnixEpoch = wasm.set_utc_full_year(event1, 1975);

                assert.equal(eventMsFromUnixEpoch, event2.getTime());
                assert.equal(event1.getTime(), event2.valueOf());
                assert.equal(event1.getUTCFullYear(), 1975);
            }
        "#,
        )
        .test()
}

#[test]
fn set_utc_hours() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::Date;

            #[wasm_bindgen]
            pub fn set_utc_hours(this: &Date, hours: u32) -> f64 {
                this.set_utc_hours(hours)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let event1 = new Date('August 19, 1975 23:15:30 GMT-3:00');
                let event2 = new Date('August 20, 1975 23:15:30 GMT');

                let eventMsFromUnixEpoch = wasm.set_utc_hours(event1, 23);

                assert.equal(eventMsFromUnixEpoch, event2.getTime());
                assert.equal(event1.getTime(), event2.valueOf());
                assert.equal(event1.getUTCHours(), 23);
            }
        "#,
        )
        .test()
}

#[test]
fn set_utc_milliseconds() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::Date;

            #[wasm_bindgen]
            pub fn set_utc_milliseconds(this: &Date, milliseconds: u32) -> f64 {
                this.set_utc_milliseconds(milliseconds)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let event1 = new Date('August 19, 1975 23:15:30 GMT-3:00');
                let event2 = new Date('August 20, 1975 02:15:30.420Z GMT');

                let eventMsFromUnixEpoch = wasm.set_utc_milliseconds(event1, 420);

                assert.equal(eventMsFromUnixEpoch, event2.getTime());
                assert.equal(event1.getTime(), event2.valueOf());
                assert.equal(event1.getUTCMilliseconds(), 420);
            }
        "#,
        )
        .test()
}

#[test]
fn set_utc_minutes() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::Date;

            #[wasm_bindgen]
            pub fn set_utc_minutes(this: &Date, minutes: u32) -> f64 {
                this.set_utc_minutes(minutes)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let event1 = new Date('December 31, 1975, 23:15:30 GMT-3:00');
                let event2 = new Date('January 01, 1976 02:25:30 GMT');

                let eventMsFromUnixEpoch = wasm.set_utc_minutes(event1, 25);

                assert.equal(eventMsFromUnixEpoch, event2.getTime());
                assert.equal(event1.getTime(), event2.valueOf());
                assert.equal(event1.getUTCMinutes(), 25);
            }
        "#,
        )
        .test()
}

#[test]
fn set_utc_month() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::Date;

            #[wasm_bindgen]
            pub fn set_utc_month(this: &Date, minutes: u32) -> f64 {
                this.set_utc_month(minutes)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let event1 = new Date('December 31, 1975 23:15:30 GMT-3:00');
                let event2 = new Date('December 01, 1976 02:15:30 GMT');

                let eventMsFromUnixEpoch = wasm.set_utc_month(event1, 11);

                assert.equal(eventMsFromUnixEpoch, event2.getTime());
                assert.equal(event1.getTime(), event2.valueOf());
                assert.equal(event1.getUTCMonth(), 11);
            }
        "#,
        )
        .test()
}

#[test]
fn set_utc_seconds() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js::Date;

            #[wasm_bindgen]
            pub fn set_utc_seconds(this: &Date, seconds: u32) -> f64 {
                this.set_utc_seconds(seconds)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let event1 = new Date('December 31, 1975 23:15:30 GMT-3:00');
                let event2 = new Date('January 01, 1976 02:15:39 GMT');

                let eventMsFromUnixEpoch = wasm.set_utc_seconds(event1, 39);

                assert.equal(eventMsFromUnixEpoch, event2.getTime());
                assert.equal(event1.getTime(), event2.valueOf());
                assert.equal(event1.getUTCSeconds(), 39);
            }
        "#,
        )
        .test()
}

#[test]
fn to_date_string() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

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
            #![feature(use_extern_macros)]

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
            #![feature(use_extern_macros)]

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
            #![feature(use_extern_macros)]

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
            #![feature(use_extern_macros)]

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
            #![feature(use_extern_macros)]

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
            #![feature(use_extern_macros)]

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
            #![feature(use_extern_macros)]

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
            #![feature(use_extern_macros)]

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
            #![feature(use_extern_macros)]

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
            #![feature(use_extern_macros)]

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
