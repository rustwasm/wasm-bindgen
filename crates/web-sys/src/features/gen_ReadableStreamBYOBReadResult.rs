#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ReadableStreamBYOBReadResult)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ReadableStreamBYOBReadResult` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamBYOBReadResult`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type ReadableStreamBYOBReadResult;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (structural , method , getter , js_class = "ReadableStreamBYOBReadResult" , js_name = value)]
    #[doc = "Getter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBReader/read)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamBYOBReadResult`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn value(this: &ReadableStreamBYOBReadResult) -> JsValue;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (structural , method , getter , js_class = "ReadableStreamBYOBReadResult" , js_name = done)]
    #[doc = "Getter for the `done` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBReader/read)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamBYOBReadResult`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn done(this: &ReadableStreamBYOBReadResult) -> bool;
}
