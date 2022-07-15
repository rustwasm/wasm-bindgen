use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn exported() -> Result<String, JsValue> {
    Err(JsValue::from(5i32))
}
