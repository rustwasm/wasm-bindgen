// FLAGS: --target=web

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn module_() -> JsValue {
    wasm_bindgen::module()
}

#[wasm_bindgen]
pub fn exports() -> JsValue {
    wasm_bindgen::exports()
}

#[wasm_bindgen]
pub fn memory_() -> JsValue {
    wasm_bindgen::memory()
}

#[wasm_bindgen]
pub fn is_memory_shared() -> bool {
    wasm_bindgen::is_memory_shared()
}

#[wasm_bindgen]
pub fn function_table() -> JsValue {
    wasm_bindgen::function_table()
}
