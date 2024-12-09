use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "number | string")]
    type CustomType;
}

#[wasm_bindgen]
pub fn single(a: CustomType) {}

#[wasm_bindgen]
pub fn slice(a: Vec<CustomType>) {}
