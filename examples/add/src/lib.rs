use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    lol as u32
}

#[wasm_bindgen]
pub enum Enum {
    A,
    B,
}

#[wasm_bindgen]
pub struct Rust {}

#[wasm_bindgen]
pub fn wut(
    // anyref
    _: &JsValue,
    _: JsValue,
    // rust
    _: &Rust,
    _: Rust,
    _: Enum,
    _: bool,
    _: char,
    // numbers
    _: f32,
    _: f64,
    _: i8,
    _: u8,
    _: i16,
    _: u16,
    _: i32,
    _: u32,
    _: i64,
    _: u64,
    // slices
    _: &[u8],
    _: &[i8],
    _: &[u16],
    _: &[i16],
    _: &[u32],
    _: &[i32],
    _: &[u64],
    _: &[i64],
    _: &[f32],
    _: &[f64],
    // vectors
    _: Vec<u8>,
    _: Vec<i8>,
    _: Vec<u16>,
    _: Vec<i16>,
    _: Vec<u32>,
    _: Vec<i32>,
    _: Vec<u64>,
    _: Vec<i64>,
    _: Vec<f32>,
    _: Vec<f64>,
    // option float
    _: Option<f32>,
    _: Option<f64>,
    // option integer
    _: Option<i8>,
    _: Option<u8>,
    _: Option<i16>,
    _: Option<u16>,
    _: Option<i32>,
    _: Option<u32>,
    _: Option<i64>,
    _: Option<u64>,
    // option misc
    _: Option<bool>,
    _: Option<char>,
    _: Option<Enum>,
    _: Option<Rust>,
    // option vectors
    _: Option<Vec<u8>>,
    _: Option<Vec<i8>>,
    _: Option<Vec<u16>>,
    _: Option<Vec<i16>>,
    _: Option<Vec<u32>>,
    _: Option<Vec<i32>>,
    _: Option<Vec<u64>>,
    _: Option<Vec<i64>>,
    _: Option<Vec<f32>>,
    _: Option<Vec<f64>>,
) {
}

#[wasm_bindgen]
pub fn goo(x: u32) {
    unsafe {
        std::mem::transmute::<u32, fn()>(x)();
    }
}

#[wasm_bindgen]
pub fn r1() -> Rust {
    loop {}
}
#[wasm_bindgen]
pub fn r2() -> Vec<u32> {
    loop {}
}
#[wasm_bindgen]
pub fn r3() -> JsValue {
    loop {}
}
#[wasm_bindgen]
pub fn r4() -> i8 {
    loop {}
}
#[wasm_bindgen]
pub fn r5() -> u8 {
    loop {}
}
#[wasm_bindgen]
pub fn r6() -> i16 {
    loop {}
}
#[wasm_bindgen]
pub fn r7() -> u16 {
    loop {}
}
#[wasm_bindgen]
pub fn r8() -> i32 {
    loop {}
}
#[wasm_bindgen]
pub fn r9() -> u32 {
    loop {}
}
#[wasm_bindgen]
pub fn r10() -> i64 {
    loop {}
}
#[wasm_bindgen]
pub fn r11() -> u64 {
    loop {}
}
#[wasm_bindgen]
pub fn r12() -> f32 {
    loop {}
}
#[wasm_bindgen]
pub fn r13() -> f64 {
    loop {}
}
#[wasm_bindgen]
pub fn r14() -> bool {
    loop {}
}
#[wasm_bindgen]
pub fn r15() -> char {
    loop {}
}
#[wasm_bindgen]
pub fn r16() -> Enum {
    loop {}
}
#[wasm_bindgen]
pub fn r17() -> Option<Vec<u32>> {
    loop {}
}
#[wasm_bindgen]
pub fn r18() -> Option<i32> {
    loop {}
}
#[wasm_bindgen]
pub fn r19() -> Option<bool> {
    loop {}
}
#[wasm_bindgen]
pub fn r20() -> Option<char> {
    loop {}
}
#[wasm_bindgen]
pub fn r21() -> Option<Enum> {
    loop {}
}
#[wasm_bindgen]
pub fn r22() -> Option<Rust> {
    loop {}
}

#[wasm_bindgen]
extern "C" {
    pub fn lol(
        // anyref
        _: &JsValue,
        _: JsValue,
        // rust
        // _: &Rust,
        _: Rust,
        _: Enum,
        _: bool,
        _: char,
        // numbers
        _: f32,
        _: f64,
        _: i8,
        _: u8,
        _: i16,
        _: u16,
        _: i32,
        _: u32,
        _: i64,
        _: u64,
        // slices
        _: &[u8],
        _: &[i8],
        _: &[u16],
        _: &[i16],
        _: &[u32],
        _: &[i32],
        _: &[u64],
        _: &[i64],
        _: &[f32],
        _: &[f64],
        // vectors
        _: Vec<u8>,
        _: Vec<i8>,
        _: Vec<u16>,
        _: Vec<i16>,
        _: Vec<u32>,
        _: Vec<i32>,
        _: Vec<u64>,
        _: Vec<i64>,
        _: Vec<f32>,
        _: Vec<f64>,
        // option float
        _: Option<f32>,
        _: Option<f64>,
        // option integer
        _: Option<i8>,
        _: Option<u8>,
        _: Option<i16>,
        _: Option<u16>,
        _: Option<i32>,
        _: Option<u32>,
        _: Option<i64>,
        _: Option<u64>,
        // option misc
        _: Option<bool>,
        _: Option<char>,
        _: Option<Enum>,
        _: Option<Rust>,
        // option vectors
        _: Option<Vec<u8>>,
        _: Option<Vec<i8>>,
        _: Option<Vec<u16>>,
        _: Option<Vec<i16>>,
        _: Option<Vec<u32>>,
        _: Option<Vec<i32>>,
        _: Option<Vec<u64>>,
        _: Option<Vec<i64>>,
        _: Option<Vec<f32>>,
        _: Option<Vec<f64>>,
        // option slices
        _: Option<&[u8]>,
        _: Option<&[i8]>,
        _: Option<&[u16]>,
        _: Option<&[i16]>,
        _: Option<&[u32]>,
        _: Option<&[i32]>,
        _: Option<&[u64]>,
        _: Option<&[i64]>,
        _: Option<&[f32]>,
        _: Option<&[f64]>,
        // closures
        _: &dyn Fn(),
        _: &mut dyn FnMut(),
        _: &Closure<dyn Fn()>,
        _: &Closure<dyn FnMut()>,
    );
}

macro_rules! t {
    ($($n:ident : $t:ty,)*) => (
        $(
            #[wasm_bindgen]
            pub fn $n() -> u32 {
                #[wasm_bindgen]
                extern "C" {
                    #[wasm_bindgen(js_namespace = nowhere)]
                    fn $n() -> $t;
                }
                return $n as u32;
            }

        )*
    )
}

t! {
    x1: i8,
    x2: u8,
    x3: i16,
    x4: u16,
    x5: i32,
    x6: u32,
    x7: i64,
    x8: u64,
    x9: f32,
    x10: f64,
    x11: Rust,
    x12: Vec<u32>,
    x13: JsValue,
    x14: bool,
    x15: char,
    x16: Enum,
    x17: Option<Vec<u32>>,
    x18: Option<i32>,
    x19: Option<char>,
    x20: Option<bool>,
    x21: Option<Rust>,
    x22: Option<Enum>,
}
