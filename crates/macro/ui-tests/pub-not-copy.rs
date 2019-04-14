#![crate_type = "rlib"]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct A {
    pub field: String,
}
