use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn fn_with_dup_attr1(
    #[wasm_bindgen(js_name = "firstArg", js_name = "anotherFirstArg")] arg1: u32,
) -> JsValue {
    arg1.into()
}

#[wasm_bindgen]
pub async fn fn_with_dup_attr2(
    #[wasm_bindgen(unchecked_param_type = "number", unchecked_param_type = "bigint")] arg1: u32,
) -> JsValue {
    arg1.into()
}

#[wasm_bindgen]
pub struct A {}

#[wasm_bindgen]
impl A {
    #[wasm_bindgen]
    pub async fn fn_with_dup_attr3(
        #[wasm_bindgen(param_description = "some number")]
        #[wasm_bindgen(param_description = "some other description")] arg1: u32,
    ) -> JsValue {
        arg1.into()
    }
}

fn main() {}
