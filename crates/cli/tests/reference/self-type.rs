use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Test;

#[wasm_bindgen]
impl Test {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Test {
        Test
    }

    pub fn consume_self(self) {}
    pub fn ref_self(&self) {}
    pub fn ref_mut_self(&mut self) {}

    pub fn self_Self(self: Self) {}
    pub fn self_ref_Self(self: &Self) {}
    pub fn self_ref_mut_Self(self: &mut Self) {}
}
