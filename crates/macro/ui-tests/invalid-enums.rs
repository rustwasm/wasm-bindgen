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

#[wasm_bindgen]
pub enum H {
    A = 1,
    B = 1, // collision
}

#[wasm_bindgen]
pub enum I {
    A = 4294967294, // = u32::MAX - 1
    B, // would be u32::MAX
    C, // would be u32::MAX + 1
}

#[wasm_bindgen]
pub enum J {
    A, // = 0
    B = 0, // collision
}

#[wasm_bindgen]
pub enum K {
    A = 3,
    B = 2,
    C, // = 3 -> collision
}

fn main() {}
