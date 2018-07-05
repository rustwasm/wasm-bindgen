#![allow(non_snake_case)]

use project;

#[test]
fn length() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn string_length(this: &js::JsString) -> u32 {
                this.length()
            }

        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let x = 'Mozilla';
                assert.equal(wasm.string_length(x), 7);

                let empty = '';
                assert.equal(wasm.string_length(empty), 0);
            }
        "#,
        )
        .test()
}

#[test]
fn char_at() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn string_char_at(this: &js::JsString, index: u32) -> js::JsString {
                this.char_at(index)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            var anyString = 'Brave new world';

            export function test() {
                assert.equal(wasm.string_char_at(anyString, 0), "B");
                assert.equal(wasm.string_char_at(anyString, 999), "");
            }
        "#,
        )
        .test()
}

#[test]
fn char_code_at() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn string_char_code_at(this: &js::JsString, index: u32) -> js::Number {
                this.char_code_at(index)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            var anyString = 'Brave new world';

            export function test() {
                assert.equal(wasm.string_char_code_at(anyString, 0), 66);
                assert.ok(isNaN(wasm.string_char_code_at(anyString, 999)));
            }
        "#,
        )
        .test()
}

#[test]
fn code_point_at() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn string_code_point_at(this: &js::JsString, pos: u32) -> JsValue {
                this.code_point_at(pos)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.string_code_point_at('ABC', 1), 66);
                assert.equal(wasm.string_code_point_at('\uD800\uDC00', 0), 65536);
                assert.equal(wasm.string_code_point_at('XYZ', 42), undefined);
            }
        "#,
        )
        .test()
}

#[test]
fn concat() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn string_concat(this: &js::JsString, string_2: &js::JsString) -> js::JsString {
                this.concat(string_2)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                // TODO: Implement ability to receive multiple optional arguments
                assert.equal(wasm.string_concat('Hello ', 'World'), 'Hello World');
                assert.equal(wasm.string_concat('foo', {}), 'foo[object Object]');
                assert.equal(wasm.string_concat('foo', []), 'foo');
                assert.equal(wasm.string_concat('foo', null), 'foonull');
                assert.equal(wasm.string_concat('foo', true), 'footrue');
                assert.equal(wasm.string_concat('foo', 1234), 'foo1234');
            }
        "#,
        )
        .test()
}

#[test]
fn includes() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn string_includes(this: &js::JsString, search_value: &js::JsString, position: i32) -> bool {
                this.includes(search_value, position)
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let str = "Blue Whale";

                // TODO: remove second parameter once we have optional parameters
                assert.equal(wasm.string_includes(str, 'Blue', 0), true);
                assert.equal(wasm.string_includes(str, 'Blute', 0), false);
                assert.equal(wasm.string_includes(str, 'Whale', 0), true);
                assert.equal(wasm.string_includes(str, 'Whale', 5), true);
                assert.equal(wasm.string_includes(str, 'Whale', 7), false);
                assert.equal(wasm.string_includes(str, '', 0), true);
                assert.equal(wasm.string_includes(str, '', 16), true);
            }
        "#)
        .test()
}

#[test]
fn index_of() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn string_index_of(this: &js::JsString, search_value: &js::JsString, from_index: i32) -> i32 {
                this.index_of(search_value, from_index)
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let str = "Blue Whale";

                // TODO: remove second parameter once we have optional parameters
                assert.equal(wasm.string_index_of(str, 'Blue', 0), 0);
                // TODO: remove second parameter once we have optional parameters
                assert.equal(wasm.string_index_of(str, 'Blute', 0), -1);
                assert.equal(wasm.string_index_of(str, 'Whale', 0), 5);
                assert.equal(wasm.string_index_of(str, 'Whale', 5), 5);
                assert.equal(wasm.string_index_of(str, 'Whale', 7), -1);
                // TODO: remove second parameter once we have optional parameters
                assert.equal(wasm.string_index_of(str, '', 0), 0);
                assert.equal(wasm.string_index_of(str, '', 9), 9);
                assert.equal(wasm.string_index_of(str, '', 10), 10);
                assert.equal(wasm.string_index_of(str, '', 11), 10);
            }
        "#)
        .test()
}

#[test]
fn slice() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn create_slice(this: &js::JsString, start: u32, end: u32) -> js::JsString {
                this.slice(start, end)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let characters = "acxn18";
                let subset = wasm.create_slice(characters, 1, 3);

                assert.equal(subset, "cx");
            }
        "#,
        )
        .test()
}

#[test]
fn starts_with() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn string_starts_with(this: &js::JsString, search_string: &js::JsString, position: u32) -> bool {
                this.starts_with(search_string, position)
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let str = "To be, or not to be, that is the question.";

                // TODO: remove second parameter for both assertions once we have optional parameters
                assert.ok(wasm.string_starts_with(str, 'To be', 0));
                assert.ok(!wasm.string_starts_with(str, 'not to be', 0));
                assert.ok(wasm.string_starts_with(str, 'not to be', 10));
            }
        "#)
        .test()
}

#[test]
fn substring() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn string_substring(this: &js::JsString, index_start: u32, index_end: u32) -> js::JsString {
                this.substring(index_start, index_end)
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let anyString = "Mozilla";

                assert.equal(wasm.string_substring(anyString, 0, 1), 'M');
                assert.equal(wasm.string_substring(anyString, 1, 0), 'M');

                assert.equal(wasm.string_substring(anyString, 0, 6), 'Mozill');

                // TODO: Add test once we have optional parameters
                // assert.equal(wasm.string_substring(anyString, 4), 'lla');
                assert.equal(wasm.string_substring(anyString, 4, 7), 'lla');
                assert.equal(wasm.string_substring(anyString, 7, 4), 'lla');

                assert.equal(wasm.string_substring(anyString, 0, 7), 'Mozilla');
                assert.equal(wasm.string_substring(anyString, 0, 10), 'Mozilla');
            }
        "#)
        .test()
}

#[test]
fn substr() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn create_substr(this: &js::JsString, start: i32, length: i32) -> js::JsString {
                this.substr(start, length)
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let aString = "Mozilla";

                assert.equal(wasm.create_substr(aString, 0, 1), "M");
                assert.equal(wasm.create_substr(aString, 1, 0), "");
                assert.equal(wasm.create_substr(aString, -1, 1), "a");
                assert.equal(wasm.create_substr(aString, 1, -1), "");
                // TODO: Uncomment and test these assertions, once we have support for optional parameters
                // assert.equal(wasm.create_substr(aString, -3), "lla");
                // assert.equal(wasm.create_substr(aString, 1), "ozilla");
                assert.equal(wasm.create_substr(aString, -20, 2), "Mo");
                assert.equal(wasm.create_substr(aString, 20, 2), "");
            }
        "#)
        .test()
}

#[test]
fn to_lower_case() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn string_to_lower_case(this: &js::JsString) -> js::JsString {
                this.to_lower_case()
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.string_to_lower_case("Mozilla"), "mozilla");
            }
        "#)
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
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn string_to_string(this: &js::JsString) -> js::JsString {
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
                let greeting = 'Hello world!';
                assert.equal(wasm.string_to_string(greeting), 'Hello world!');
            }
        "#,
        )
        .test()
}

#[test]
fn to_upper_case() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn string_to_upper_case(this: &js::JsString) -> js::JsString {
                this.to_upper_case()
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.string_to_upper_case("Mozilla"), "MOZILLA");
            }
        "#)
        .test()
}

#[test]
fn trim() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn string_trim(this: &js::JsString) -> js::JsString {
                this.trim()
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.string_trim('   foo  '), 'foo');
                // Another example of .trim() removing whitespace from just one side.
                assert.equal(wasm.string_trim('foo   '), 'foo');
            }
        "#,
        )
        .test()
}

#[test]
fn trim_end_and_trim_right() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn string_trim_end(this: &js::JsString) -> js::JsString {
                this.trim_end()
            }

            #[wasm_bindgen]
            pub fn string_trim_right(this: &js::JsString) -> js::JsString {
                this.trim_right()
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let greeting = '   Hello world!   ';
                let trimmed = '   Hello world!';
                assert.equal(wasm.string_trim_end(greeting), trimmed);
                assert.equal(wasm.string_trim_right(greeting), trimmed);
            }
        "#,
        )
        .test()
}

#[test]
fn trim_start_and_trim_left() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn string_trim_start(this: &js::JsString) -> js::JsString {
                this.trim_start()
            }

            #[wasm_bindgen]
            pub fn string_trim_left(this: &js::JsString) -> js::JsString {
                this.trim_left()
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                let greeting = '   Hello world!   ';
                let trimmed = 'Hello world!   ';
                assert.equal(wasm.string_trim_start(greeting), trimmed);
                assert.equal(wasm.string_trim_left(greeting), trimmed);
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
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn string_value_of(this: &js::JsString) -> js::JsString {
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
                let greeting = new String('Hello world!');
                assert.equal(wasm.string_value_of(greeting), 'Hello world!');
            }
        "#,
        )
        .test()
}
