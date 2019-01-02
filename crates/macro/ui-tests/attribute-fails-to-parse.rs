#![crate_type = "rlib"]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(nonsense)]
pub fn foo() {}
