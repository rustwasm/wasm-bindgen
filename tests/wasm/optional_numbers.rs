use wasm_bindgen_test::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "tests/wasm/optional_numbers.js", version = "*")]
extern {
    fn i32_js_identity(a: Option<i32>) -> Option<i32>;
    fn u32_js_identity(a: Option<u32>) -> Option<u32>;
    fn isize_js_identity(a: Option<isize>) -> Option<isize>;
    fn usize_js_identity(a: Option<usize>) -> Option<usize>;
    fn f32_js_identity(a: Option<f32>) -> Option<f32>;
    fn f64_js_identity(a: Option<f64>) -> Option<f64>;
    fn i8_js_identity(a: Option<i8>) -> Option<i8>;
    fn u8_js_identity(a: Option<u8>) -> Option<u8>;
    fn i16_js_identity(a: Option<i16>) -> Option<i16>;
    fn u16_js_identity(a: Option<u16>) -> Option<u16>;
    fn i64_js_identity(a: Option<i64>) -> Option<i64>;
    fn u64_js_identity(a: Option<u64>) -> Option<u64>;
    fn bool_js_identity(a: Option<bool>) -> Option<bool>;

    fn test_works();
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

#[wasm_bindgen]
pub fn bool_none() -> Option<bool> { None }
#[wasm_bindgen]
pub fn bool_false() -> Option<bool> { Some(false) }
#[wasm_bindgen]
pub fn bool_true() -> Option<bool> { Some(true) }
#[wasm_bindgen]
pub fn bool_identity(a: Option<bool>) -> Option<bool> { bool_js_identity(a) }

#[wasm_bindgen_test]
fn works() {
    test_works();
}
