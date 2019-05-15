use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn foo() {}

#[wasm_bindgen(start)]
pub fn foo2(x: u32) {}

#[wasm_bindgen(start)]
pub fn foo3<T>() {}

fn main() {}
