use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ExportedNamedStruct {
    pub inner: u32,
}

#[wasm_bindgen]
impl ExportedNamedStruct {
    #[wasm_bindgen(constructor)]
    pub fn new(inner: u32) -> WasmType<ExportedNamedStruct> {
        instantiate! { ExportedNamedStruct { inner } }
    }
}

#[wasm_bindgen]
pub fn named_struct_by_value(x: WasmType<ExportedNamedStruct>) {}

#[wasm_bindgen]
pub fn named_struct_by_shared_ref(x: &ExportedNamedStruct) {}

#[wasm_bindgen]
pub fn named_struct_by_exclusive_ref(x: &mut ExportedNamedStruct) {}

#[wasm_bindgen]
pub fn return_named_struct(inner: u32) -> WasmType<ExportedNamedStruct> {
    ExportedNamedStruct::new(inner)
}

#[wasm_bindgen]
pub struct ExportedTupleStruct(pub u32, pub u32);

#[wasm_bindgen]
impl ExportedTupleStruct {
    #[wasm_bindgen(constructor)]
    pub fn new(x: u32, y: u32) -> WasmType<ExportedTupleStruct> {
        instantiate! { ExportedTupleStruct(x, y) }
    }
}

#[wasm_bindgen]
pub fn return_tuple_struct(x: u32, y: u32) -> WasmType<ExportedTupleStruct> {
    ExportedTupleStruct::new(x, y)
}
