#![deny(unused_variables)]

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn fn_with_attr(
    a: u32,
    #[wasm_bindgen(optional)] b: JsValue,
    c: bool,
) -> Result<(), JsValue> {
    Ok(())
}

fn main() {}
