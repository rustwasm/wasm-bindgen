use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const TS_INTERFACE_EXPORT: &'static str = r"
  interface Height { height: number; }
";

#[wasm_bindgen(typescript_custom_section)]
const TS_INTERFACE_EXPORT_NON_LITERAL: &'static str = std::concat!(
    "interface Name { name: NameString; }\n",
    std::include_str!("./data/custom_section_intf.ts")
);

#[wasm_bindgen]
pub struct Person {
    pub height: u32,
}

#[wasm_bindgen]
impl Person {
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        "Alice".into()
    }
}
