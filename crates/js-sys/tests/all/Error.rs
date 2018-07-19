#![allow(non_snake_case)]

use project;

#[test]
fn new() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;
            use wasm_bindgen::js::Error;

            #[wasm_bindgen]
            pub fn new_error(message: &js::JsString) -> Error {
                Error::new(message)
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                const message = 'any error message';
                const error = wasm.new_error(message);

                assert.equal(error.message, message);
            }
        "#)
        .test()
}

#[test]
fn message() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;
            use wasm_bindgen::js::Error;

            #[wasm_bindgen]
            pub fn error_message(this: &Error) -> js::JsString {
                this.message()
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                const message = 'any error message';
                const error = new Error(message);

                assert.equal(wasm.error_message(error), message);
            }
        "#)
        .test()
}

#[test]
fn set_message() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;
            use wasm_bindgen::js::Error;

            #[wasm_bindgen]
            pub fn error_set_message(this: &Error, message: &js::JsString) {
                this.set_message(message);
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                const message = 'any error message';
                const error = new Error();
                wasm.error_set_message(error, message);

                assert.equal(error.message, message);
            }
        "#)
        .test()
}

#[test]
fn name() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;
            use wasm_bindgen::js::Error;

            #[wasm_bindgen]
            pub fn error_name(this: &Error) -> js::JsString {
                this.name()
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                const name = 'any error name';
                const error = new Error();
                error.name = name;

                assert.equal(wasm.error_name(error), name);
            }
        "#)
        .test()
}

#[test]
fn set_name() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;
            use wasm_bindgen::js::Error;

            #[wasm_bindgen]
            pub fn error_set_name(this: &Error, name: &js::JsString) {
                this.set_name(name);
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                const name = 'any error name';
                const error = new Error();
                wasm.error_set_name(error, name);

                assert.equal(error.name, name);
            }
        "#)
        .test()
}

#[test]
fn to_string() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;
            use wasm_bindgen::js::Error;

            #[wasm_bindgen]
            pub fn error_to_string(this: &Error) -> js::JsString {
                this.to_string()
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                const error = new Error('error message 1');

                assert.equal(wasm.error_to_string(error), 'Error: error message 1');

                error.name = undefined;
                assert.equal(wasm.error_to_string(error), 'Error: error message 1');

                error.name = 'error_name_1';
                assert.equal(wasm.error_to_string(error), 'error_name_1: error message 1');

                error.message = undefined;
                assert.equal(wasm.error_to_string(error), 'error_name_1');

                error.name = 'error_name_2';
                assert.equal(wasm.error_to_string(error), 'error_name_2');
            }
        "#)
        .test()
}
