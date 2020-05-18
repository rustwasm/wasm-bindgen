// enable-externref

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch)]
    fn foo() -> Result<(), JsValue>;
}

#[wasm_bindgen]
pub fn exported() -> Result<(), JsValue> {
    foo()
}
