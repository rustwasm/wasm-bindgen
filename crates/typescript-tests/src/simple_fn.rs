use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(_: &str) {}

#[wasm_bindgen]
pub fn take_and_return_bool(_: bool) -> bool {
    true
}

#[wasm_bindgen(variadic)]
pub fn variadic_function(arr: &JsValue) -> JsValue {
    arr.into()
}
