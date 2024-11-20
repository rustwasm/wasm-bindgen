use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const TS_INTERFACE_EXPORT: &'static str = r"
  interface Height { height: number; }
";

#[wasm_bindgen(typescript_custom_section)]
const TS_INTERFACE_EXPORT1: &'static str = include_str!("./custom_section_types.d.ts");

#[allow(dead_code)]
const TS_INTERFACE_EXPORT2: &str = "interface Person2 { height: number; }";
#[wasm_bindgen(typescript_custom_section)]
const _: &str = TS_INTERFACE_EXPORT2;

#[wasm_bindgen]
pub struct Person {
    pub height: u32,
}

#[wasm_bindgen]
impl Person {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { height: 170 }
    }
}
