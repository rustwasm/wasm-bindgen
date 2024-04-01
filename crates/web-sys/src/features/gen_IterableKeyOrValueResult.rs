#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = IterableKeyOrValueResult)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IterableKeyOrValueResult` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IterableKeyOrValueResult`*"]
    pub type IterableKeyOrValueResult;
    #[wasm_bindgen(method, setter = "done")]
    fn done_shim(this: &IterableKeyOrValueResult, val: bool);
    #[wasm_bindgen(method, setter = "value")]
    fn value_shim(this: &IterableKeyOrValueResult, val: &::wasm_bindgen::JsValue);
}
impl IterableKeyOrValueResult {
    #[doc = "Construct a new `IterableKeyOrValueResult`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IterableKeyOrValueResult`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `done` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IterableKeyOrValueResult`*"]
    pub fn done(&mut self, val: bool) -> &mut Self {
        self.done_shim(val);
        self
    }
    #[doc = "Change the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IterableKeyOrValueResult`*"]
    pub fn value(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.value_shim(val);
        self
    }
}
impl Default for IterableKeyOrValueResult {
    fn default() -> Self {
        Self::new()
    }
}
