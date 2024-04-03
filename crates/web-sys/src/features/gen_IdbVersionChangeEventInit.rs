#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = IDBVersionChangeEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdbVersionChangeEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbVersionChangeEventInit`*"]
    pub type IdbVersionChangeEventInit;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn bubbles_shim(this: &IdbVersionChangeEventInit, val: bool);
    #[wasm_bindgen(method, setter = "cancelable")]
    fn cancelable_shim(this: &IdbVersionChangeEventInit, val: bool);
    #[wasm_bindgen(method, setter = "composed")]
    fn composed_shim(this: &IdbVersionChangeEventInit, val: bool);
    #[wasm_bindgen(method, setter = "newVersion")]
    fn new_version_shim(this: &IdbVersionChangeEventInit, val: Option<f64>);
    #[wasm_bindgen(method, setter = "oldVersion")]
    fn old_version_shim(this: &IdbVersionChangeEventInit, val: f64);
}
impl IdbVersionChangeEventInit {
    #[doc = "Construct a new `IdbVersionChangeEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbVersionChangeEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbVersionChangeEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbVersionChangeEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbVersionChangeEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.composed_shim(val);
        self
    }
    #[doc = "Change the `newVersion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbVersionChangeEventInit`*"]
    pub fn new_version(&mut self, val: Option<f64>) -> &mut Self {
        self.new_version_shim(val);
        self
    }
    #[doc = "Change the `oldVersion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbVersionChangeEventInit`*"]
    pub fn old_version(&mut self, val: f64) -> &mut Self {
        self.old_version_shim(val);
        self
    }
}
impl Default for IdbVersionChangeEventInit {
    fn default() -> Self {
        Self::new()
    }
}
