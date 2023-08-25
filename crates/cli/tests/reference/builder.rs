use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ClassBuilder(());

#[wasm_bindgen]
impl ClassBuilder {
    pub fn builder() -> ClassBuilder {
        ClassBuilder(())
    }
}
