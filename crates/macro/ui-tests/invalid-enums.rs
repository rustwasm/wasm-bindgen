#![feature(use_extern_macros)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
enum A {}

#[wasm_bindgen]
pub enum B {
    D(u32),
}

#[wasm_bindgen]
pub enum C {
    X = 1 + 3,
}

#[wasm_bindgen]
pub enum D {
    X = 4294967296,
}
