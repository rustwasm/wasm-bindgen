use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn async_greet(_: String) {}

#[wasm_bindgen]
pub async fn async_take_and_return_bool(_: bool) -> bool {
    true
}
