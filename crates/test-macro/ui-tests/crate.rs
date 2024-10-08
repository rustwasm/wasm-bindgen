#![no_implicit_prelude]

extern crate wasm_bindgen_test_macro;
//
use wasm_bindgen_test_macro::wasm_bindgen_test;

pub mod wasm {
    pub extern crate wasm_bindgen_test as test;
}

#[wasm_bindgen_test(crate = ::wasm_bindgen_test)]
fn success_1() {}

#[wasm_bindgen_test(crate = crate::wasm::test)]
fn success_2() {}

#[wasm_bindgen_test(crate(foo))]
fn failure_1() {}

fn main() {}
