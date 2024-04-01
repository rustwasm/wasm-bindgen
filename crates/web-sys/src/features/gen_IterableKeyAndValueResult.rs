#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = IterableKeyAndValueResult)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IterableKeyAndValueResult` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IterableKeyAndValueResult`*"]
    pub type IterableKeyAndValueResult;
    #[wasm_bindgen(method, setter = "done")]
    fn done_shim(this: &IterableKeyAndValueResult, val: bool);
    #[wasm_bindgen(method, setter = "value")]
    fn value_shim(this: &IterableKeyAndValueResult, val: &::wasm_bindgen::JsValue);
}
impl IterableKeyAndValueResult {
    #[doc = "Construct a new `IterableKeyAndValueResult`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IterableKeyAndValueResult`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `done` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IterableKeyAndValueResult`*"]
    pub fn done(&mut self, val: bool) -> &mut Self {
        self.done_shim(val);
        self
    }
    #[doc = "Change the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IterableKeyAndValueResult`*"]
    pub fn value(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.value_shim(val);
        self
    }
}
impl Default for IterableKeyAndValueResult {
    fn default() -> Self {
        Self::new()
    }
}
