use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[rustfmt::skip]
#[wasm_bindgen(module = "tests/wasm/return_result.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    fn call_exports() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    fn return_value() -> Result<JsValue, JsValue>;
    #[wasm_bindgen(catch)]
    fn return_some() -> Result<Option<JsValue>, JsValue>;
    #[wasm_bindgen(catch)]
    fn return_none() -> Result<Option<JsValue>, JsValue>;
}

#[wasm_bindgen_test]
fn smoke() {
    call_exports().unwrap();
}

#[wasm_bindgen]
pub fn nothing() -> Result<(), JsValue> {
    Ok(())
}

#[wasm_bindgen]
pub fn return_3() -> Result<u32, JsValue> {
    Ok(3)
}

#[wasm_bindgen]
pub fn return_4() -> Result<JsValue, JsValue> {
    Ok(4.into())
}

#[wasm_bindgen]
pub fn return_option() -> Result<JsValue, JsValue> {
    Ok(Some(42).into())
}

#[wasm_bindgen]
pub fn return_option_some() -> Result<Option<JsValue>, JsValue> {
    Ok(Some(42.into()))
}

#[wasm_bindgen]
pub fn return_option_none() -> Result<Option<JsValue>, JsValue> {
    Ok(None)
}

#[wasm_bindgen_test]
fn test_return_value() {
    assert_eq!(
        return_value().unwrap().as_string(),
        Some(String::from("ok"))
    )
}

#[wasm_bindgen_test]
fn test_return_some() {
    assert_eq!(
        return_some().unwrap().unwrap().as_string(),
        Some(String::from("ok"))
    )
}

#[wasm_bindgen_test]
fn test_return_none() {
    assert_eq!(return_none().unwrap().is_none(), true)
}
