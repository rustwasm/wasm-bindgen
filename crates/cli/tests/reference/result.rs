use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn result_string() -> Result<String, JsValue> {
    Err(JsValue::from(5i32))
}

#[wasm_bindgen]
pub fn result_void() -> Result<(), JsError> {
    Err(JsError::new("message"))
}

#[wasm_bindgen]
pub fn result_i32() -> Result<i32, JsError> {
    Ok(1)
}
