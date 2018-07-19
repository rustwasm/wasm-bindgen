#![allow(non_snake_case)]

use super::project;

#[test]
fn abs() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn abs(x: f64) -> f64 {
                js::Math::abs(x)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.abs(-32), Math.abs(-32));
                assert.equal(wasm.abs(32), 32);
                assert.equal(wasm.abs(-4.7), Math.abs(-4.7));
            }
        "#,
        )
        .test()
}

#[test]
fn acos() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn acos(x: f64) -> f64 {
                js::Math::acos(x)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.acos(-1), Math.PI);
                assert.equal(wasm.acos(0.5), 1.0471975511965979);
                assert.ok(Number.isNaN(wasm.acos(2)));
            }
        "#,
        )
        .test()
}

#[test]
fn acosh() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn acosh(x: f64) -> f64 {
                js::Math::acosh(x)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.acosh(1), 0);
                assert.equal(wasm.acosh(2), Math.acosh(2));
                assert.ok(Number.isNaN(wasm.acosh(0.5)));
            }
        "#,
        )
        .test()
}

#[test]
fn asin() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn asin(x: f64) -> f64 {
                js::Math::asin(x)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.asin(1), Math.asin(1));
                assert.equal(wasm.asin(0.5), Math.asin(0.5));
                assert.ok(Number.isNaN(wasm.asin(2)));
            }
        "#,
        )
        .test()
}

#[test]
fn asinh() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn asinh(x: f64) -> f64 {
                js::Math::asinh(x)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.asinh(1), Math.asinh(1));
                assert.equal(wasm.asinh(0.5), Math.asinh(0.5));
            }
        "#,
        )
        .test()
}

#[test]
fn atan() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn atan(x: f64) -> f64 {
                js::Math::atan(x)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.atan(1), Math.atan(1));
                assert.equal(wasm.atan(0.5), Math.atan(0.5));
            }
        "#,
        )
        .test()
}

#[test]
fn atan2() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn atan2(y: f64, x: f64) -> f64 {
                js::Math::atan2(y, x)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.atan2(1, 2), Math.atan2(1, 2));
                assert.equal(wasm.atan2(0.7, 3.8), Math.atan2(0.7, 3.8));
            }
        "#,
        )
        .test()
}

#[test]
fn atanh() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn atanh(x: f64) -> f64 {
                js::Math::atanh(x)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.atanh(1), Math.atanh(1));
                assert.equal(wasm.atanh(0.5), Math.atanh(0.5));
                assert.ok(Number.isNaN(wasm.atanh(2)));
            }
        "#,
        )
        .test()
}

#[test]
fn cbrt() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn cbrt(x: f64) -> f64 {
                js::Math::cbrt(x)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.cbrt(27), 3);
                assert.equal(wasm.cbrt(12.3), Math.cbrt(12.3));
            }
        "#,
        )
        .test()
}

#[test]
fn ceil() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn ceil(x: f64) -> i32 {
                js::Math::ceil(x)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.ceil(1.1), 2);
                assert.equal(wasm.ceil(-1.1), -1);
            }
        "#,
        )
        .test()
}

#[test]
fn clz32() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn clz32(x: i32) -> u32 {
                js::Math::clz32(x)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.clz32(1), 31);
                assert.equal(wasm.clz32(1000), 22);
            }
        "#,
        )
        .test()
}

#[test]
fn cos() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn cos(x: f64) -> f64 {
                js::Math::cos(x)
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.cos(0), 1);
                assert.equal(wasm.cos(1.5), Math.cos(1.5));
            }
        "#)
        .test()
}

#[test]
fn cosh() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn cosh(x: f64) -> f64 {
                js::Math::cosh(x)
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.cosh(0), 1);
                assert.equal(wasm.cosh(2), 3.7621956910836314);
            }
        "#)
        .test()
}

#[test]
fn exp() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn exp(x: f64) -> f64 {
                js::Math::exp(x)
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.exp(0), 1);
                assert.equal(wasm.exp(-1), 0.36787944117144233);
                assert.equal(wasm.exp(2), 7.38905609893065);
            }
        "#)
        .test()
}

#[test]
fn expm1() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn expm1(x: f64) -> f64 {
                js::Math::expm1(x)
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.expm1(0), 0);
                assert.equal(wasm.expm1(1), 1.718281828459045);
                assert.equal(wasm.expm1(-1), -0.6321205588285577);
                assert.equal(wasm.expm1(2), 6.38905609893065);
            }
        "#)
        .test()
}

#[test]
fn floor() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn floor(x: f64) -> i32 {
                js::Math::floor(x)
            }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.floor(5.95), 5);
                assert.equal(wasm.floor(-5.05), -6);
            }
        "#,
        )
        .test()
}

#[test]
fn fround() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn fround(x: f64) -> f32 {
                js::Math::fround(x)
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.fround(5.5), 5.5);
                assert.equal(wasm.fround(5.05), 5.050000190734863);
                assert.equal(wasm.fround(5), 5);
                assert.equal(wasm.fround(-5.05), -5.050000190734863);
            }
        "#)
        .test()
}

#[test]
fn imul() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn imul(x: i32, y:i32) -> i32 {
                js::Math::imul(x, y)
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.imul(3, 4), 12);
                assert.equal(wasm.imul(-5, 12), -60);
                assert.equal(wasm.imul(0xffffffff, 5), Math.imul(0xffffffff, 5));
            }
        "#)
        .test()
}

#[test]
fn log() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn log(x: f64) -> f64 {
                js::Math::log(x)
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.log(8) / wasm.log(2), 3);
                assert.equal(wasm.log(625) / wasm.log(5), 4);
            }
        "#)
        .test()
}

#[test]
fn log10() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn log10(x: f64) -> f64 {
                js::Math::log10(x)
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.log10(100000), 5);
                assert.equal(wasm.log10(1), 0);
                assert.equal(wasm.log10(2), 0.3010299956639812);
            }
        "#)
        .test()
}

#[test]
fn log1p() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn log1p(x: f64) -> f64 {
                js::Math::log1p(x)
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.log1p(1), 0.6931471805599453);
                assert.equal(wasm.log1p(0), 0);
                assert.equal(wasm.log1p(-1), -Infinity);
                assert.ok(isNaN(wasm.log1p(-2)));
            }
        "#)
        .test()
}

#[test]
fn log2() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn log2(x: f64) -> f64 {
                js::Math::log2(x)
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.log2(3), 1.584962500721156);
                assert.equal(wasm.log2(2), 1);
                assert.equal(wasm.log2(1), 0);
                assert.equal(wasm.log2(0), -Infinity);
            }
        "#)
        .test()
}

#[test]
fn pow() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn pow(base: f64, exponent: f64) -> f64 {
                js::Math::pow(base, exponent)
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.pow(7, 2), 49);
                assert.equal(wasm.pow(3.8, 0.5), Math.pow(3.8, 0.5));
                assert.ok(Number.isNaN(wasm.pow(-2, 0.5)));
            }
        "#)
        .test()
}

#[test]
fn random() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn random() -> f64 {
                js::Math::random()
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.ok(wasm.random() < 1);
                assert.ok(wasm.random() >= 0);
            }
        "#)
        .test()
}

#[test]
fn round() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn round(x: f64) -> i32 {
                js::Math::round(x)
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.round(20.49), 20);
                assert.equal(wasm.round(20.5), 21);
                assert.equal(wasm.round(42), 42);
                assert.equal(wasm.round(-20.5), -20);
                assert.equal(wasm.round(-20.51), -21);
            }
        "#)
        .test()
}

#[test]
fn sign() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn sign(x: f64) -> f64 {
                js::Math::sign(x)
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.sign(3), 1);
                assert.equal(wasm.sign(-3), -1);
                assert.equal(wasm.sign(2.3), 1);
                assert.equal(wasm.sign(0), 0);
                assert.ok(Number.isNaN(wasm.sign(NaN)));
            }
        "#)
        .test()
}

#[test]
fn sin() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn sin(x: f64) -> f64 {
                js::Math::sin(x)
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.sin(0), 0);
                assert.equal(wasm.sin(1), Math.sin(1));
                assert.equal(wasm.sin(Math.PI / 2), 1);
            }
        "#)
        .test()
}

#[test]
fn sinh() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn sinh(x: f64) -> f64 {
                js::Math::sinh(x)
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.sinh(0), 0);
                assert.equal(wasm.sinh(1), Math.sinh(1));
                assert.equal(wasm.sinh(2.3), Math.sinh(2.3));
            }
        "#)
        .test()
}

#[test]
fn sqrt() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn sqrt(x: f64) -> f64 {
                js::Math::sqrt(x)
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.sqrt(9), 3);
                assert.equal(wasm.sqrt(2), Math.sqrt(2));
                assert.equal(wasm.sqrt(42.42), Math.sqrt(42.42));
                assert.equal(wasm.sqrt(1), 1);
                assert.ok(Number.isNaN(wasm.sqrt(-1)));
            }
        "#)
        .test()
}

#[test]
fn tan() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn tan(x: f64) -> f64 {
                js::Math::tan(x)
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.tan(0), 0);
                assert.equal(wasm.tan(1), Math.tan(1));
                assert.equal(wasm.tan(0.5), Math.tan(0.5));
            }
        "#)
        .test()
}

#[test]
fn tanh() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn tanh(x: f64) -> f64 {
                js::Math::tanh(x)
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.tanh(0), 0);
                assert.equal(wasm.tanh(1), Math.tanh(1));
                assert.equal(wasm.tanh(0.5), Math.tanh(0.5));
            }
        "#)
        .test()
}

#[test]
fn trunc() {
    project()
        .file("src/lib.rs", r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn trunc(x: f64) -> i32 {
                js::Math::trunc(x)
            }
        "#)
        .file("test.js", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.trunc(13.37), 13);
                assert.equal(wasm.trunc(42.84), 42);
                assert.equal(wasm.trunc(0.123), 0);
                assert.equal(wasm.trunc(-0.123), 0);
            }
        "#)
        .test()
}
