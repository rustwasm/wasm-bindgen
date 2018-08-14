use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ExportedRustType {
    inner: u32,
}

#[wasm_bindgen]
pub fn exported_type_by_value(x: ExportedRustType) {}

#[wasm_bindgen]
pub fn exported_type_by_shared_ref(x: &ExportedRustType) {}

#[wasm_bindgen]
pub fn exported_type_by_exclusive_ref(x: &mut ExportedRustType) {}

#[wasm_bindgen]
pub fn return_exported_type() -> ExportedRustType {
    unimplemented!()
}
