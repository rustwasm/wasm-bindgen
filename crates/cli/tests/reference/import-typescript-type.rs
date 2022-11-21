use wasm_bindgen::prelude::*;

#[wasm_bindgen(raw_module = "somepackage")]
extern "C" {
    pub type SomeType;
}

#[wasm_bindgen]
pub fn something(a: &SomeType) {
    drop(a);
}
