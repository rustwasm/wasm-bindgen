#![cfg(target_arch = "wasm32")]
#![feature(use_extern_macros)]

extern crate wasm_bindgen_test;
extern crate wasm_bindgen;

use wasm_bindgen_test::*;
use wasm_bindgen::prelude::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen]
pub struct ConsumeRetString;

#[wasm_bindgen]
impl ConsumeRetString {
    // https://github.com/rustwasm/wasm-bindgen/issues/329#issuecomment-411082013
    //
    // This used to cause two `const ptr = ...` declarations, which is invalid
    // JS.
    pub fn consume(self) -> String { String::new() }
}

#[wasm_bindgen_test]
fn works() {
    ConsumeRetString.consume();
}
