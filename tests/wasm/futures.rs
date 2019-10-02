use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/futures.js")]
extern "C" {
    fn call_exports() -> js_sys::Promise;
}

#[wasm_bindgen_test]
async fn smoke() {
    wasm_bindgen_futures::JsFuture::from(call_exports()).await.unwrap();
}

#[wasm_bindgen]
pub async fn async_do_nothing() {}

#[wasm_bindgen]
pub async fn async_return_1() -> JsValue {
    1.into()
}

#[wasm_bindgen]
pub async fn async_return_2() -> u32 {
    2
}

#[wasm_bindgen]
pub async fn async_nothing_again() -> Result<(), JsValue> {
    Ok(())
}

#[wasm_bindgen]
pub async fn async_return_3() -> Result<u32, JsValue> {
    Ok(3)
}

#[wasm_bindgen]
pub async fn async_return_4() -> Result<JsValue, JsValue> {
    Ok(4.into())
}

#[wasm_bindgen]
pub struct AsyncCustomReturn {
    pub val: u32,
}

#[wasm_bindgen]
impl AsyncCustomReturn {
    #[wasm_bindgen(constructor)]
    pub fn new(val: u32) -> WasmType<AsyncCustomReturn> {
        instantiate! {
            AsyncCustomReturn { val }
        }
    }
}

#[wasm_bindgen]
pub async fn async_return_5() -> WasmType<AsyncCustomReturn> {
    AsyncCustomReturn::new(5)
}

#[wasm_bindgen]
pub async fn async_return_6() -> Result<WasmType<AsyncCustomReturn>, JsValue> {
    Ok(AsyncCustomReturn::new(6))
}

#[wasm_bindgen]
pub async fn async_return_7() -> Result<WasmType<AsyncCustomReturn>, u32> {
    Ok(AsyncCustomReturn::new(7))
}

#[wasm_bindgen]
pub async fn async_return_8() -> Result<WasmType<AsyncCustomReturn>, WasmType<AsyncCustomReturn>> {
    Ok(AsyncCustomReturn::new(8))
}

#[wasm_bindgen]
pub async fn async_throw() -> Result<(), js_sys::Error> {
    Err(js_sys::Error::new("async message"))
}
