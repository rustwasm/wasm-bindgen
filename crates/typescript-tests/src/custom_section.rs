use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const TS_INTERFACE_EXPORT: &'static str = r"
  interface Height { height: number; }
";

#[wasm_bindgen]
pub struct Person {
    pub height: u32,
}
