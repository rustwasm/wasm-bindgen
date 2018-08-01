#![feature(use_extern_macros)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(x)]
pub fn foo() {}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(y)]
    fn bar();

    #[wasm_bindgen z]
    fn bar();

    #[wasm_bindgen(z2) x]
    fn bar();

    #[wasm_bindgen {  }]
    fn bar();
}
