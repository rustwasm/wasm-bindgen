use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn fn_with_illegal_char_attr1(
    #[wasm_bindgen(unchecked_param_type = "abcd */firstArg")] arg1: u32,
) -> JsValue {
    arg1.into()
}

#[wasm_bindgen]
pub async fn fn_with_illegal_char_attr2(
    #[wasm_bindgen(unchecked_param_type = "num*/ber")] arg1: u32,
) -> JsValue {
    arg1.into()
}

#[wasm_bindgen]
pub async fn fn_with_illegal_char_attr3(
    #[wasm_bindgen(param_description = "/* some description */")] arg1: u32,
) -> JsValue {
    arg1.into()
}

#[wasm_bindgen(return_description = "*/ some description")]
pub async fn fn_with_illegal_char_attr4(arg1: u32) -> JsValue {
    arg1.into()
}

#[wasm_bindgen(unchecked_return_type = "number */ abcd")]
pub async fn fn_with_illegal_char_attr5(arg1: u32) -> JsValue {
    arg1.into()
}

fn main() {}
