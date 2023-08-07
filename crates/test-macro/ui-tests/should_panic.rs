#![no_implicit_prelude]

extern crate wasm_bindgen_test_macro;

use wasm_bindgen_test_macro::wasm_bindgen_test;

#[wasm_bindgen_test]
#[should_panic]
fn success_1() {}

#[wasm_bindgen_test]
#[should_panic = "test"]
fn success_2() {}

#[wasm_bindgen_test]
#[should_panic(expected = "test")]
fn success_3() {}

#[wasm_bindgen_test]
#[should_panic]
async fn async_success_1() {}

#[wasm_bindgen_test]
#[should_panic = "test"]
async fn async_success_2() {}

#[wasm_bindgen_test]
#[should_panic(expected = "test")]
async fn async_success_3() {}

#[wasm_bindgen_test]
#[should_panic::error]
fn fail_1() {}

#[wasm_bindgen_test]
#[should_panic = 42]
fn fail_2() {}

#[wasm_bindgen_test]
#[should_panic[]]
fn fail_3() {}

#[wasm_bindgen_test]
#[should_panic(42)]
fn fail_4() {}

#[wasm_bindgen_test]
#[should_panic(test)]
fn fail_5() {}

#[wasm_bindgen_test]
#[should_panic(expected)]
fn fail_6() {}

#[wasm_bindgen_test]
#[should_panic(expected::error)]
fn fail_7() {}

#[wasm_bindgen_test]
#[should_panic(expected =)]
fn fail_8() {}

#[wasm_bindgen_test]
#[should_panic(expected = 5)]
fn fail_9() {}

#[wasm_bindgen_test]
#[should_panic = "test"]
#[should_panic = "test"]
fn fail_10() {}

fn main() {}
