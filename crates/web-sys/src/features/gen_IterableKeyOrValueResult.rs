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
    #[wasm_bindgen(method, getter = "done")]
    fn done_shim(this: &IterableKeyOrValueResult) -> bool;
    #[wasm_bindgen(method, setter = "done")]
    fn set_done_shim(this: &IterableKeyOrValueResult, val: bool);
    #[wasm_bindgen(method, getter = "value")]
    fn value_shim(this: &IterableKeyOrValueResult) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "value")]
    fn set_value_shim(this: &IterableKeyOrValueResult, val: &::wasm_bindgen::JsValue);
}
#[doc = "The trait to access properties on the `IterableKeyOrValueResult` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `IterableKeyOrValueResult`*"]
pub trait IterableKeyOrValueResultGetters {
    #[doc = "Get the `done` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IterableKeyOrValueResult`*"]
    fn done(&self) -> bool;
    #[doc = "Get the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IterableKeyOrValueResult`*"]
    fn value(&self) -> &::wasm_bindgen::JsValue;
}
impl IterableKeyOrValueResultGetters for IterableKeyOrValueResult {
    fn done(&self) -> bool {
        self.done_shim()
    }
    fn value(&self) -> &::wasm_bindgen::JsValue {
        self.value_shim()
    }
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
        self.set_done_shim(val);
        self
    }
    #[doc = "Change the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IterableKeyOrValueResult`*"]
    pub fn value(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_value_shim(val);
        self
    }
}
impl Default for IterableKeyOrValueResult {
    fn default() -> Self {
        Self::new()
    }
}
