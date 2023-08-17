use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ClassBuilder(String);

#[wasm_bindgen]
impl ClassBuilder {
    pub fn builder() -> Self {
        ClassBuilder(String::from("Test"))
    }
}
