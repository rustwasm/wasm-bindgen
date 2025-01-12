use wasm_bindgen::prelude::*;

#[wasm_bindgen(unchecked_return_type = "number")]
pub fn no_ret_fn1() {}

#[wasm_bindgen(return_description = "some description")]
pub async fn no_ret_fn2() {}

#[wasm_bindgen]
pub struct A {}

#[wasm_bindgen]
impl A {
    #[wasm_bindgen(unchecked_return_type = "number")]
    pub async fn no_ret_method1() {}

    #[wasm_bindgen(return_description = "some description")]
    pub fn no_ret_method2() {}
}

fn main() {}
