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

#[wasm_bindgen]
pub enum E {
    A = 1,
    B = "foo",
}

#[wasm_bindgen]
pub enum F {
    A = "foo",
    B = 1,
}

#[wasm_bindgen]
enum G {
    A = "foo",
    B = "bar",
    C,
}

fn main() {}
