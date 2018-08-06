#![feature(use_extern_macros)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "tests/wasm/duplicate_deps.js", version = "*")]
extern {
    fn foo();
}

pub fn test() {
    foo();
}
