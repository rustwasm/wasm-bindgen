use super::project;

#[test]
fn works() {
    project()
        .requires_bigint()
        .file(
            "src/lib.rs",
            r#"
            #![feature(use_extern_macros)]

            extern crate wasm_bindgen;

            use wasm_bindgen::prelude::*;

            #[wasm_bindgen(module = "./test")]
            extern {
                fn i32_js_identity(a: Option<i32>) -> Option<i32>;
            }
            #[wasm_bindgen]
            pub fn i32_none() -> Option<i32> { None }
            #[wasm_bindgen]
            pub fn i32_zero() -> Option<i32> { Some(0) }
            #[wasm_bindgen]
            pub fn i32_one() -> Option<i32> { Some(1) }
            #[wasm_bindgen]
            pub fn i32_neg_one() -> Option<i32> { Some(-1) }
            #[wasm_bindgen]
            pub fn i32_max() -> Option<i32> { Some(i32::max_value()) }
            #[wasm_bindgen]
            pub fn i32_min() -> Option<i32> { Some(i32::min_value()) }
            #[wasm_bindgen]
            pub fn i32_identity(a: Option<i32>) -> Option<i32> { i32_js_identity(a) }

            #[wasm_bindgen(module = "./test")]
            extern {
                fn u32_js_identity(a: Option<u32>) -> Option<u32>;
            }
            #[wasm_bindgen]
            pub fn u32_none() -> Option<u32> { None }
            #[wasm_bindgen]
            pub fn u32_zero() -> Option<u32> { Some(0) }
            #[wasm_bindgen]
            pub fn u32_one() -> Option<u32> { Some(1) }
            #[wasm_bindgen]
            pub fn u32_max() -> Option<u32> { Some(u32::max_value()) }
            #[wasm_bindgen]
            pub fn u32_min() -> Option<u32> { Some(u32::min_value()) }
            #[wasm_bindgen]
            pub fn u32_identity(a: Option<u32>) -> Option<u32> { u32_js_identity(a) }

            #[wasm_bindgen(module = "./test")]
            extern {
                fn isize_js_identity(a: Option<isize>) -> Option<isize>;
            }
            #[wasm_bindgen]
            pub fn isize_none() -> Option<isize> { None }
            #[wasm_bindgen]
            pub fn isize_zero() -> Option<isize> { Some(0) }
            #[wasm_bindgen]
            pub fn isize_one() -> Option<isize> { Some(1) }
            #[wasm_bindgen]
            pub fn isize_neg_one() -> Option<isize> { Some(-1) }
            #[wasm_bindgen]
            pub fn isize_max() -> Option<isize> { Some(isize::max_value()) }
            #[wasm_bindgen]
            pub fn isize_min() -> Option<isize> { Some(isize::min_value()) }
            #[wasm_bindgen]
            pub fn isize_identity(a: Option<isize>) -> Option<isize> { isize_js_identity(a) }

            #[wasm_bindgen(module = "./test")]
            extern {
                fn usize_js_identity(a: Option<usize>) -> Option<usize>;
            }
            #[wasm_bindgen]
            pub fn usize_none() -> Option<usize> { None }
            #[wasm_bindgen]
            pub fn usize_zero() -> Option<usize> { Some(0) }
            #[wasm_bindgen]
            pub fn usize_one() -> Option<usize> { Some(1) }
            #[wasm_bindgen]
            pub fn usize_max() -> Option<usize> { Some(usize::max_value()) }
            #[wasm_bindgen]
            pub fn usize_min() -> Option<usize> { Some(usize::min_value()) }
            #[wasm_bindgen]
            pub fn usize_identity(a: Option<usize>) -> Option<usize> { usize_js_identity(a) }

            #[wasm_bindgen(module = "./test")]
            extern {
                fn f32_js_identity(a: Option<f32>) -> Option<f32>;
            }
            #[wasm_bindgen]
            pub fn f32_none() -> Option<f32> { None }
            #[wasm_bindgen]
            pub fn f32_zero() -> Option<f32> { Some(0f32) }
            #[wasm_bindgen]
            pub fn f32_one() -> Option<f32> { Some(1f32) }
            #[wasm_bindgen]
            pub fn f32_neg_one() -> Option<f32> { Some(-1f32) }
            #[wasm_bindgen]
            pub fn f32_identity(a: Option<f32>) -> Option<f32> { f32_js_identity(a) }

            #[wasm_bindgen(module = "./test")]
            extern {
                fn f64_js_identity(a: Option<f64>) -> Option<f64>;
            }
            #[wasm_bindgen]
            pub fn f64_none() -> Option<f64> { None }
            #[wasm_bindgen]
            pub fn f64_zero() -> Option<f64> { Some(0f64) }
            #[wasm_bindgen]
            pub fn f64_one() -> Option<f64> { Some(1f64) }
            #[wasm_bindgen]
            pub fn f64_neg_one() -> Option<f64> { Some(-1f64) }
            #[wasm_bindgen]
            pub fn f64_identity(a: Option<f64>) -> Option<f64> { f64_js_identity(a) }

            #[wasm_bindgen(module = "./test")]
            extern {
                fn i8_js_identity(a: Option<i8>) -> Option<i8>;
            }
            #[wasm_bindgen]
            pub fn i8_none() -> Option<i8> { None }
            #[wasm_bindgen]
            pub fn i8_zero() -> Option<i8> { Some(0) }
            #[wasm_bindgen]
            pub fn i8_one() -> Option<i8> { Some(1) }
            #[wasm_bindgen]
            pub fn i8_neg_one() -> Option<i8> { Some(-1) }
            #[wasm_bindgen]
            pub fn i8_max() -> Option<i8> { Some(i8::max_value()) }
            #[wasm_bindgen]
            pub fn i8_min() -> Option<i8> { Some(i8::min_value()) }
            #[wasm_bindgen]
            pub fn i8_identity(a: Option<i8>) -> Option<i8> { i8_js_identity(a) }

            #[wasm_bindgen(module = "./test")]
            extern {
                fn u8_js_identity(a: Option<u8>) -> Option<u8>;
            }
            #[wasm_bindgen]
            pub fn u8_none() -> Option<u8> { None }
            #[wasm_bindgen]
            pub fn u8_zero() -> Option<u8> { Some(0) }
            #[wasm_bindgen]
            pub fn u8_one() -> Option<u8> { Some(1) }
            #[wasm_bindgen]
            pub fn u8_max() -> Option<u8> { Some(u8::max_value()) }
            #[wasm_bindgen]
            pub fn u8_min() -> Option<u8> { Some(u8::min_value()) }
            #[wasm_bindgen]
            pub fn u8_identity(a: Option<u8>) -> Option<u8> { u8_js_identity(a) }

            #[wasm_bindgen(module = "./test")]
            extern {
                fn i16_js_identity(a: Option<i16>) -> Option<i16>;
            }
            #[wasm_bindgen]
            pub fn i16_none() -> Option<i16> { None }
            #[wasm_bindgen]
            pub fn i16_zero() -> Option<i16> { Some(0) }
            #[wasm_bindgen]
            pub fn i16_one() -> Option<i16> { Some(1) }
            #[wasm_bindgen]
            pub fn i16_neg_one() -> Option<i16> { Some(-1) }
            #[wasm_bindgen]
            pub fn i16_max() -> Option<i16> { Some(i16::max_value()) }
            #[wasm_bindgen]
            pub fn i16_min() -> Option<i16> { Some(i16::min_value()) }
            #[wasm_bindgen]
            pub fn i16_identity(a: Option<i16>) -> Option<i16> { i16_js_identity(a) }

            #[wasm_bindgen(module = "./test")]
            extern {
                fn u16_js_identity(a: Option<u16>) -> Option<u16>;
            }
            #[wasm_bindgen]
            pub fn u16_none() -> Option<u16> { None }
            #[wasm_bindgen]
            pub fn u16_zero() -> Option<u16> { Some(0) }
            #[wasm_bindgen]
            pub fn u16_one() -> Option<u16> { Some(1) }
            #[wasm_bindgen]
            pub fn u16_max() -> Option<u16> { Some(u16::max_value()) }
            #[wasm_bindgen]
            pub fn u16_min() -> Option<u16> { Some(u16::min_value()) }
            #[wasm_bindgen]
            pub fn u16_identity(a: Option<u16>) -> Option<u16> { u16_js_identity(a) }

            #[wasm_bindgen(module = "./test")]
            extern {
                fn i64_js_identity(a: Option<i64>) -> Option<i64>;
            }
            #[wasm_bindgen]
            pub fn i64_none() -> Option<i64> { None }
            #[wasm_bindgen]
            pub fn i64_zero() -> Option<i64> { Some(0) }
            #[wasm_bindgen]
            pub fn i64_one() -> Option<i64> { Some(1) }
            #[wasm_bindgen]
            pub fn i64_neg_one() -> Option<i64> { Some(-1) }
            #[wasm_bindgen]
            pub fn i64_max() -> Option<i64> { Some(i64::max_value()) }
            #[wasm_bindgen]
            pub fn i64_min() -> Option<i64> { Some(i64::min_value()) }
            #[wasm_bindgen]
            pub fn i64_identity(a: Option<i64>) -> Option<i64> { i64_js_identity(a) }

            #[wasm_bindgen(module = "./test")]
            extern {
                fn u64_js_identity(a: Option<u64>) -> Option<u64>;
            }
            #[wasm_bindgen]
            pub fn u64_none() -> Option<u64> { None }
            #[wasm_bindgen]
            pub fn u64_zero() -> Option<u64> { Some(0) }
            #[wasm_bindgen]
            pub fn u64_one() -> Option<u64> { Some(1) }
            #[wasm_bindgen]
            pub fn u64_max() -> Option<u64> { Some(u64::max_value()) }
            #[wasm_bindgen]
            pub fn u64_min() -> Option<u64> { Some(u64::min_value()) }
            #[wasm_bindgen]
            pub fn u64_identity(a: Option<u64>) -> Option<u64> { u64_js_identity(a) }
        "#,
        )
        .file(
            "test.js",
            r#"
            import * as wasm from './out';

            function assertEq(a, b) {
              console.log(a, '?=', b);
              if (a === b)
                return;
              throw new Error('not equal');
            }

            export function test() {
              assertEq(wasm.i32_identity(wasm.i32_none()), undefined);
              assertEq(wasm.i32_identity(wasm.i32_zero()), 0);
              assertEq(wasm.i32_identity(wasm.i32_one()), 1);
              assertEq(wasm.i32_identity(wasm.i32_neg_one()), -1);
              assertEq(wasm.i32_identity(wasm.i32_max()), 2147483647);
              assertEq(wasm.i32_identity(wasm.i32_min()), -2147483648);

              assertEq(wasm.u32_identity(wasm.u32_none()), undefined);
              assertEq(wasm.u32_identity(wasm.u32_zero()), 0);
              assertEq(wasm.u32_identity(wasm.u32_one()), 1);
              assertEq(wasm.u32_identity(wasm.u32_max()), 4294967295);
              assertEq(wasm.u32_identity(wasm.u32_min()), 0);

              assertEq(wasm.isize_identity(wasm.isize_none()), undefined);
              assertEq(wasm.isize_identity(wasm.isize_zero()), 0);
              assertEq(wasm.isize_identity(wasm.isize_one()), 1);
              assertEq(wasm.isize_identity(wasm.isize_neg_one()), -1);
              assertEq(wasm.isize_identity(wasm.isize_max()), 2147483647);
              assertEq(wasm.isize_identity(wasm.isize_min()), -2147483648);

              assertEq(wasm.usize_identity(wasm.usize_none()), undefined);
              assertEq(wasm.usize_identity(wasm.usize_zero()), 0);
              assertEq(wasm.usize_identity(wasm.usize_one()), 1);
              assertEq(wasm.usize_identity(wasm.usize_max()), 4294967295);
              assertEq(wasm.usize_identity(wasm.usize_min()), 0);

              assertEq(wasm.f32_identity(wasm.f32_none()), undefined);
              assertEq(wasm.f32_identity(wasm.f32_zero()), 0);
              assertEq(wasm.f32_identity(wasm.f32_one()), 1);
              assertEq(wasm.f32_identity(wasm.f32_neg_one()), -1);

              assertEq(wasm.f64_identity(wasm.f64_none()), undefined);
              assertEq(wasm.f64_identity(wasm.f64_zero()), 0);
              assertEq(wasm.f64_identity(wasm.f64_one()), 1);
              assertEq(wasm.f64_identity(wasm.f64_neg_one()), -1);

              assertEq(wasm.i8_identity(wasm.i8_none()), undefined);
              assertEq(wasm.i8_identity(wasm.i8_zero()), 0);
              assertEq(wasm.i8_identity(wasm.i8_one()), 1);
              assertEq(wasm.i8_identity(wasm.i8_neg_one()), -1);
              assertEq(wasm.i8_identity(wasm.i8_max()), 127);
              assertEq(wasm.i8_identity(wasm.i8_min()), -128);

              assertEq(wasm.u8_identity(wasm.u8_none()), undefined);
              assertEq(wasm.u8_identity(wasm.u8_zero()), 0);
              assertEq(wasm.u8_identity(wasm.u8_one()), 1);
              assertEq(wasm.u8_identity(wasm.u8_max()), 255);
              assertEq(wasm.u8_identity(wasm.u8_min()), 0);

              assertEq(wasm.i16_identity(wasm.i16_none()), undefined);
              assertEq(wasm.i16_identity(wasm.i16_zero()), 0);
              assertEq(wasm.i16_identity(wasm.i16_one()), 1);
              assertEq(wasm.i16_identity(wasm.i16_neg_one()), -1);
              assertEq(wasm.i16_identity(wasm.i16_max()), 32767);
              assertEq(wasm.i16_identity(wasm.i16_min()), -32768);

              assertEq(wasm.u16_identity(wasm.u16_none()), undefined);
              assertEq(wasm.u16_identity(wasm.u16_zero()), 0);
              assertEq(wasm.u16_identity(wasm.u16_one()), 1);
              assertEq(wasm.u16_identity(wasm.u16_max()), 65535);
              assertEq(wasm.u16_identity(wasm.u16_min()), 0);

              assertEq(wasm.i64_identity(wasm.i64_none()), undefined);
              assertEq(wasm.i64_identity(wasm.i64_zero()), BigInt("0"));
              assertEq(wasm.i64_identity(wasm.i64_one()), BigInt("1"));
              assertEq(wasm.i64_identity(wasm.i64_neg_one()), BigInt("-1"));
              assertEq(wasm.i64_identity(wasm.i64_max()), BigInt("9223372036854775807"));
              assertEq(wasm.i64_identity(wasm.i64_min()), BigInt("-9223372036854775808"));

              assertEq(wasm.u64_identity(wasm.u64_none()), undefined);
              assertEq(wasm.u64_identity(wasm.u64_zero()), BigInt("0"));
              assertEq(wasm.u64_identity(wasm.u64_one()), BigInt("1"));
              assertEq(wasm.u64_identity(wasm.u64_max()), BigInt("18446744073709551615"));
              assertEq(wasm.u64_identity(wasm.u64_min()), BigInt("0"));
            }

            export function i32_js_identity(a) { return a; }
            export function u32_js_identity(a) { return a; }
            export function isize_js_identity(a) { return a; }
            export function usize_js_identity(a) { return a; }
            export function f32_js_identity(a) { return a; }
            export function f64_js_identity(a) { return a; }
            export function i8_js_identity(a) { return a; }
            export function u8_js_identity(a) { return a; }
            export function i16_js_identity(a) { return a; }
            export function u16_js_identity(a) { return a; }
            export function i64_js_identity(a) { return a; }
            export function u64_js_identity(a) { return a; }
        "#,
        )
        .test();
}
