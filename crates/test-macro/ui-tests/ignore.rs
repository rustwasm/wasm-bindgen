#![no_implicit_prelude]

extern crate wasm_bindgen_test_macro;

use wasm_bindgen_test_macro::wasm_bindgen_test;

#[wasm_bindgen_test]
#[ignore]
fn success_1() {}

#[wasm_bindgen_test]
#[ignore = "test"]
fn success_2() {}

#[wasm_bindgen_test]
#[ignore]
async fn async_success_1() {}

#[wasm_bindgen_test]
#[ignore = "test"]
async fn async_success_2() {}

#[wasm_bindgen_test]
#[ignore::error]
fn fail_1() {}

#[wasm_bindgen_test]
#[ignore = 42]
fn fail_2() {}

#[wasm_bindgen_test]
#[ignore[]]
fn fail_3() {}

#[wasm_bindgen_test]
#[ignore(42)]
fn fail_4() {}

#[wasm_bindgen_test]
#[ignore(test)]
fn fail_5() {}

#[wasm_bindgen_test]
#[ignore("test")]
fn fail_6() {}

#[wasm_bindgen_test]
#[ignore = "test"]
#[ignore = "test"]
fn fail_7() {}

fn main() {}
