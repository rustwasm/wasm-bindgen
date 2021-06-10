#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ReadableStreamDefaultReadResult)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ReadableStreamDefaultReadResult` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamDefaultReadResult`*"]
    pub type ReadableStreamDefaultReadResult;
    # [wasm_bindgen (structural , method , getter , js_class = "ReadableStreamDefaultReadResult" , js_name = value)]
    #[doc = "Getter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultReader/read)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamDefaultReadResult`*"]
    pub fn value(this: &ReadableStreamDefaultReadResult) -> JsValue;
    # [wasm_bindgen (structural , method , getter , js_class = "ReadableStreamDefaultReadResult" , js_name = done)]
    #[doc = "Getter for the `done` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultReader/read)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamDefaultReadResult`*"]
    pub fn done(this: &ReadableStreamDefaultReadResult) -> bool;
}
