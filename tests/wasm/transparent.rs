use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/transparent.js")]
extern "C" {
    fn js_transparent_u32();
    fn js_transparent_u32_empty();
    fn js_trans_trans_u32();
}

#[repr(transparent)]
#[wasm_bindgen]
pub struct TransparentU32(u32);

#[wasm_bindgen]
pub fn make_transparent_u32(value: u32) -> TransparentU32 {
    TransparentU32(value)
}

#[wasm_bindgen]
pub fn get_transparent_u32(value: TransparentU32) -> u32 {
    value.0
}

#[wasm_bindgen_test]
fn transparent_u32() {
    js_transparent_u32();
}

#[repr(transparent)]
#[wasm_bindgen]
pub struct TransparentU32Empty(u32, ());

#[wasm_bindgen]
pub fn make_transparent_u32_empty(value: u32) -> TransparentU32Empty {
    TransparentU32Empty(value, ())
}

#[wasm_bindgen]
pub fn get_transparent_u32_empty(value: TransparentU32Empty) -> u32 {
    value.0
}

#[wasm_bindgen_test]
fn transparent_u32_empty() {
    js_transparent_u32_empty();
}

#[repr(transparent)]
#[wasm_bindgen]
pub struct TransTransU32(TransparentU32);

#[wasm_bindgen]
pub fn make_trans_trans_u32(value: u32) -> TransTransU32 {
    TransTransU32(make_transparent_u32(value))
}

#[wasm_bindgen]
pub fn get_trans_trans_u32(value: TransTransU32) -> u32 {
    get_transparent_u32(value.0)
}

#[wasm_bindgen_test]
fn trans_trans_u32() {
    js_trans_trans_u32();
}
