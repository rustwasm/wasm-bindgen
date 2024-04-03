#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = UnderlyingSink)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UnderlyingSink` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSink`*"]
    pub type UnderlyingSink;
    #[wasm_bindgen(method, setter = "abort")]
    fn abort_shim(this: &UnderlyingSink, val: &::js_sys::Function);
    #[wasm_bindgen(method, setter = "close")]
    fn close_shim(this: &UnderlyingSink, val: &::js_sys::Function);
    #[wasm_bindgen(method, setter = "start")]
    fn start_shim(this: &UnderlyingSink, val: &::js_sys::Function);
    #[wasm_bindgen(method, setter = "type")]
    fn type__shim(this: &UnderlyingSink, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "write")]
    fn write_shim(this: &UnderlyingSink, val: &::js_sys::Function);
}
impl UnderlyingSink {
    #[doc = "Construct a new `UnderlyingSink`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSink`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `abort` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSink`*"]
    pub fn abort(&mut self, val: &::js_sys::Function) -> &mut Self {
        self.abort_shim(val);
        self
    }
    #[doc = "Change the `close` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSink`*"]
    pub fn close(&mut self, val: &::js_sys::Function) -> &mut Self {
        self.close_shim(val);
        self
    }
    #[doc = "Change the `start` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSink`*"]
    pub fn start(&mut self, val: &::js_sys::Function) -> &mut Self {
        self.start_shim(val);
        self
    }
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSink`*"]
    pub fn type_(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.type__shim(val);
        self
    }
    #[doc = "Change the `write` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSink`*"]
    pub fn write(&mut self, val: &::js_sys::Function) -> &mut Self {
        self.write_shim(val);
        self
    }
}
impl Default for UnderlyingSink {
    fn default() -> Self {
        Self::new()
    }
}
