#![allow(non_snake_case)]

use super::project;


#[test]
fn abs() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn abs(number: i32) -> js::Number {
                js::Math::abs(number)
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.abs(-32), Math.abs(-32));
                assert.equal(wasm.abs(32), 32);
            }
        "#)
        .test()
}

#[test]
fn acos() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn acos(adjacent: i32, hypotenuse: i32) -> js::Number {
                js::Math::acos(adjacent, hypotenuse)
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.acos(-1, 1), Math.PI);
            }
        "#)
        .test()
}

#[test]
fn acosh() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn acosh(number: i32) -> js::Number {
                js::Math::acosh(number)
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.acosh(1), 0);
                assert.equal(wasm.acosh(2), Math.acosh(2));
            }
        "#)
        .test()
}

#[test]
fn asin() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn asin(opposite: i32, hypotenuse: i32) -> js::Number {
                js::Math::asin(opposite / hypotenuse)
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.asin(1, 1), Math.asin(1));
            }
        "#)
        .test()
}

#[test]
fn asinh() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn asinh(number: i32) -> js::Number {
                js::Math::asinh(number)
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.asinh(1), Math.asinh(1));
            }
        "#)
        .test()
}
