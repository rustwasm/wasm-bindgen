use js_sys::{Int32Array, Promise, SharedArrayBuffer};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/polyfill.js")]
extern "C" {
    #[wasm_bindgen(js_name = waitAsync)]
    pub fn wait_async(indexed_array: Int32Array, index: u32, value: i32) -> Promise;

    #[wasm_bindgen(js_name = waitAsync)]
    pub fn wait_async_with_timeout(
        indexed_array: Int32Array,
        index: u32,
        value: i32,
        timeout: f64,
    ) -> Promise;
}
