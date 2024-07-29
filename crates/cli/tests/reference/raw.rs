use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn r#test1(r#test: u32) -> u32 {
    r#test2();
    r#test
}

#[wasm_bindgen]
pub struct r#Test;

#[wasm_bindgen]
impl r#Test {
    pub fn r#test1(r#test: u32) -> Self {
        Self
    }

    pub fn r#test2(&self, r#test: u32) {}
}

#[wasm_bindgen(module = "test")]
extern "C" {
    fn r#test2() -> JsValue;
}
