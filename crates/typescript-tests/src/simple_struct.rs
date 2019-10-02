use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct A {}

#[wasm_bindgen]
impl A {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmType<A> {
        instantiate! { A {} }
    }

    pub fn other() {}

    pub fn foo(&self) {}
}
