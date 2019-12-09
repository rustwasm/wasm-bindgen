use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn foo() {}

#[wasm_bindgen(start)]
pub fn foo2(x: u32) {}

#[wasm_bindgen(start)]
pub fn foo3<T>() {}

#[wasm_bindgen(start)]
pub fn foo4() -> Result<(), JsValue> { Ok(()) }

#[wasm_bindgen(start)]
pub fn foo5() -> Result<JsValue, ()> { Err(()) }

#[wasm_bindgen(start)]
pub fn foo6() -> Result<JsValue, JsValue> { Ok(JsValue::from(1u32)) }

#[wasm_bindgen(start)]
pub async fn foo_async1() {}

#[wasm_bindgen(start)]
pub async fn foo_async2() -> Result<(), JsValue> { Ok(()) }

#[wasm_bindgen(start)]
pub async fn foo_async3() -> Result<JsValue, ()> { Err(()) }

#[wasm_bindgen(start)]
pub async fn foo_async4() -> Result<JsValue, JsValue> { Ok(JsValue::from(1u32)) }

fn main() {}
