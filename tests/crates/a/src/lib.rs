extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "tests/wasm/duplicate_deps.js")]
extern "C" {
    fn foo();
}

pub fn test() {
    foo();
}
