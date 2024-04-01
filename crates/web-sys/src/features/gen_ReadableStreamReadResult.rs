#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ReadableStreamReadResult)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ReadableStreamReadResult` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamReadResult`*"]
    pub type ReadableStreamReadResult;
    #[wasm_bindgen(method, setter = "done")]
    fn done_shim(this: &ReadableStreamReadResult, val: bool);
    #[wasm_bindgen(method, setter = "value")]
    fn value_shim(this: &ReadableStreamReadResult, val: &::wasm_bindgen::JsValue);
}
impl ReadableStreamReadResult {
    #[doc = "Construct a new `ReadableStreamReadResult`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamReadResult`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `done` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamReadResult`*"]
    pub fn done(&mut self, val: bool) -> &mut Self {
        self.done_shim(val);
        self
    }
    #[doc = "Change the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamReadResult`*"]
    pub fn value(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.value_shim(val);
        self
    }
}
impl Default for ReadableStreamReadResult {
    fn default() -> Self {
        Self::new()
    }
}
