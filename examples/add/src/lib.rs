use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct Answer(u32);

#[wasm_bindgen]
impl Answer {
    pub fn new() -> Answer {
        Answer(41)
    }
    #[wasm_bindgen(getter)]
    pub fn the_answer(self) -> u32 {
        self.0 + 1
    }
    pub fn foo(self) -> u32 {
        self.0 + 1
    }
}
