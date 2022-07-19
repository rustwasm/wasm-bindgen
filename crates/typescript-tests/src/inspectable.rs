use wasm_bindgen::prelude::*;

#[wasm_bindgen(inspectable)]
pub struct Bar {
    pub foo: i32,
    _private: i32,
}

#[wasm_bindgen]
impl Bar {
    #[wasm_bindgen(constructor)]
    pub fn new(foo: i32) -> Self {
        Self { foo, _private: 13 }
    }
}
