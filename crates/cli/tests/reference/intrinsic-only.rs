use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn causes_error() -> Result<f64, JsError> {
    Ok(1.0)
}
