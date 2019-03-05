use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(_: &str) {}

#[wasm_bindgen]
struct A {
}

#[wasm_bindgen]
impl A {
    #[wasm_bindgen(constructor)]
    pub fn new() -> A {
        A {}
    }

    pub fn other() {}

    pub fn foo(&self) {}
}
