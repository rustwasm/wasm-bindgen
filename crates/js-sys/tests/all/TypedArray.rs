#![allow(non_snake_case)]

use project;
use std::string::String;

fn new_undefined_lib(array_type: &str) -> String {
    format!(r#"
    #![feature(use_extern_macros)]

    extern crate wasm_bindgen;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::js;

    #[wasm_bindgen]
    pub fn new_array() -> js::{} {{
        js::{}::new(JsValue::undefined())
    }}
    "#, array_type, array_type)
}

fn new_undefined_test_js() -> &'static str {
    r#"
    import * as assert from "assert";
    import * as wasm from "./out";

    export function test() {
        assert.equal(wasm.new_array().length, 0);
    }
    "#
}

#[test]
fn new_Uint8Array_undefined() {
    project()
        .file("src/lib.rs", &new_undefined_lib("Uint8Array"),)
        .file("test.js", new_undefined_test_js(),)
        .test()
}

#[test]
fn new_Uint8ClampedArray_undefined() {
    project()
        .file("src/lib.rs", &new_undefined_lib("Uint8ClampedArray"),)
        .file("test.js", new_undefined_test_js(),)
        .test()
}

#[test]
fn new_Uint16Array_undefined() {
    project()
        .file("src/lib.rs", &new_undefined_lib("Uint16Array"),)
        .file("test.js", new_undefined_test_js(),)
        .test()
}

#[test]
fn new_Uint32Array_undefined() {
    project()
        .file("src/lib.rs", &new_undefined_lib("Uint32Array"),)
        .file("test.js", new_undefined_test_js(),)
        .test()
}

#[test]
fn new_Int8Array_undefined() {
    project()
        .file("src/lib.rs", &new_undefined_lib("Int8Array"),)
        .file("test.js", new_undefined_test_js(),)
        .test()
}

#[test]
fn new_Int16Array_undefined() {
    project()
        .file("src/lib.rs", &new_undefined_lib("Int16Array"),)
        .file("test.js", new_undefined_test_js(),)
        .test()
}

#[test]
fn new_Int32Array_undefined() {
    project()
        .file("src/lib.rs", &new_undefined_lib("Int32Array"),)
        .file("test.js", new_undefined_test_js(),)
        .test()
}

#[test]
fn new_Float32Array_undefined() {
    project()
        .file("src/lib.rs", &new_undefined_lib("Float32Array"),)
        .file("test.js", new_undefined_test_js(),)
        .test()
}

#[test]
fn new_Float64Array_undefined() {
    project()
        .file("src/lib.rs", &new_undefined_lib("Float64Array"),)
        .file("test.js", new_undefined_test_js(),)
        .test()
}

fn new_length_lib(array_type: &str) -> String {
    format!(r#"
    #![feature(use_extern_macros)]

    extern crate wasm_bindgen;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::js;

    #[wasm_bindgen]
    pub fn new_array() -> js::{} {{
        js::{}::new(JsValue::from_f64(4.0))
    }}
    "#, array_type, array_type)
}

fn new_length_test_js() -> &'static str {
    r#"
    import * as assert from "assert";
    import * as wasm from "./out";

    export function test() {
        assert.equal(wasm.new_array().length, 4);
    }
    "#
}

#[test]
fn new_Uint8Array_length() {
    project()
        .file("src/lib.rs", &new_length_lib("Uint8Array"),)
        .file("test.js", new_length_test_js(),)
        .test()
}

#[test]
fn new_Uint8ClampedArray_length() {
    project()
        .file("src/lib.rs", &new_length_lib("Uint8ClampedArray"),)
        .file("test.js", new_length_test_js(),)
        .test()
}

#[test]
fn new_Uint16Array_length() {
    project()
        .file("src/lib.rs", &new_length_lib("Uint16Array"),)
        .file("test.js", new_length_test_js(),)
        .test()
}

#[test]
fn new_Uint32Array_length() {
    project()
        .file("src/lib.rs", &new_length_lib("Uint32Array"),)
        .file("test.js", new_length_test_js(),)
        .test()
}

#[test]
fn new_Int8Array_length() {
    project()
        .file("src/lib.rs", &new_length_lib("Int8Array"),)
        .file("test.js", new_length_test_js(),)
        .test()
}

#[test]
fn new_Int16Array_length() {
    project()
        .file("src/lib.rs", &new_length_lib("Int16Array"),)
        .file("test.js", new_length_test_js(),)
        .test()
}

#[test]
fn new_Int32Array_length() {
    project()
        .file("src/lib.rs", &new_length_lib("Int32Array"),)
        .file("test.js", new_length_test_js(),)
        .test()
}

#[test]
fn new_Float32Array_length() {
    project()
        .file("src/lib.rs", &new_length_lib("Float32Array"),)
        .file("test.js", new_length_test_js(),)
        .test()
}

#[test]
fn new_Float64Array_length() {
    project()
        .file("src/lib.rs", &new_length_lib("Float64Array"),)
        .file("test.js", new_length_test_js(),)
        .test()
}

fn fill_lib(array_type: &str, el_type: &str) -> String {
    format!(r#"
    #![feature(use_extern_macros)]

    extern crate wasm_bindgen;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::js;

    #[wasm_bindgen]
    pub fn fill_with(this: &js::{}, value: {}, start: u32, end: u32) -> js::{} {{
        this.fill(value, start, end)
    }}
    "#, array_type, el_type, array_type)
}

fn fill_test_js(array_type: &str) -> String {
    format!(r#"
    import * as assert from "assert";
    import * as wasm from "./out";

    export function test() {{
        let characters = new {}([0, 0, 0, 0, 0, 0]);
        let subset = wasm.fill_with(characters, 1, 0, 3);

        assert.equal(subset[0], 1);
        assert.equal(subset[4], 0);
    }}
    "#, array_type)
}

#[test]
fn fill_Uint8Array() {
    project()
        .file("src/lib.rs", &fill_lib("Uint8Array", "u8"),)
        .file("test.js", &fill_test_js("Uint8Array"))
        .test()
}

#[test]
fn fill_Uint8ClampedArray() {
    project()
        .file("src/lib.rs", &fill_lib("Uint8ClampedArray", "u8"),)
        .file("test.js", &fill_test_js("Uint8ClampedArray"))
        .test()
}

#[test]
fn fill_Uint16Array() {
    project()
        .file("src/lib.rs", &fill_lib("Uint16Array", "u16"),)
        .file("test.js", &fill_test_js("Uint16Array"))
        .test()
}

#[test]
fn fill_Uint32Array() {
    project()
        .file("src/lib.rs", &fill_lib("Uint32Array", "u32"),)
        .file("test.js", &fill_test_js("Uint32Array"))
        .test()
}

#[test]
fn fill_Int8Array() {
    project()
        .file("src/lib.rs", &fill_lib("Int8Array", "i8"),)
        .file("test.js", &fill_test_js("Int8Array"))
        .test()
}

#[test]
fn fill_Int16Array() {
    project()
        .file("src/lib.rs", &fill_lib("Int16Array", "i16"),)
        .file("test.js", &fill_test_js("Int16Array"))
        .test()
}

#[test]
fn fill_Int32Array() {
    project()
        .file("src/lib.rs", &fill_lib("Int32Array", "i32"),)
        .file("test.js", &fill_test_js("Int32Array"))
        .test()
}

#[test]
fn fill_Float32Array() {
    project()
        .file("src/lib.rs", &fill_lib("Float32Array", "f32"),)
        .file("test.js", &fill_test_js("Float32Array"))
        .test()
}

#[test]
fn fill_Float64Array() {
    project()
        .file("src/lib.rs", &fill_lib("Float64Array", "f64"),)
        .file("test.js", &fill_test_js("Float64Array"))
        .test()
}
