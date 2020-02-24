use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct A {}

#[wasm_bindgen]
impl A {
    #[wasm_bindgen(constructor)]
    pub fn new() -> A {
        A {}
    }

    pub fn other() {}

    pub fn foo(&self) {}

    pub fn ret_bool(&self) -> bool {
        true
    }
    pub fn take_bool(&self, _: bool) {}
    pub fn take_many(&self, _: bool, _: f64, _: u32) {}
}
