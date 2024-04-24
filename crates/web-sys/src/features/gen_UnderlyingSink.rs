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
    #[wasm_bindgen(method, getter = "abort")]
    fn abort_shim(this: &UnderlyingSink) -> ::js_sys::Function;
    #[wasm_bindgen(method, setter = "abort")]
    fn set_abort_shim(this: &UnderlyingSink, val: &::js_sys::Function);
    #[wasm_bindgen(method, getter = "close")]
    fn close_shim(this: &UnderlyingSink) -> ::js_sys::Function;
    #[wasm_bindgen(method, setter = "close")]
    fn set_close_shim(this: &UnderlyingSink, val: &::js_sys::Function);
    #[wasm_bindgen(method, getter = "start")]
    fn start_shim(this: &UnderlyingSink) -> ::js_sys::Function;
    #[wasm_bindgen(method, setter = "start")]
    fn set_start_shim(this: &UnderlyingSink, val: &::js_sys::Function);
    #[wasm_bindgen(method, getter = "type")]
    fn type__shim(this: &UnderlyingSink) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "type")]
    fn set_type__shim(this: &UnderlyingSink, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "write")]
    fn write_shim(this: &UnderlyingSink) -> ::js_sys::Function;
    #[wasm_bindgen(method, setter = "write")]
    fn set_write_shim(this: &UnderlyingSink, val: &::js_sys::Function);
}
#[doc = "The trait to access properties on the `UnderlyingSink` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `UnderlyingSink`*"]
pub trait UnderlyingSinkGetters {
    #[doc = "Get the `abort` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSink`*"]
    fn abort(&self) -> ::js_sys::Function;
    #[doc = "Get the `close` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSink`*"]
    fn close(&self) -> ::js_sys::Function;
    #[doc = "Get the `start` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSink`*"]
    fn start(&self) -> ::js_sys::Function;
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSink`*"]
    fn type_(&self) -> ::wasm_bindgen::JsValue;
    #[doc = "Get the `write` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSink`*"]
    fn write(&self) -> ::js_sys::Function;
}
impl UnderlyingSinkGetters for UnderlyingSink {
    fn abort(&self) -> ::js_sys::Function {
        self.abort_shim()
    }
    fn close(&self) -> ::js_sys::Function {
        self.close_shim()
    }
    fn start(&self) -> ::js_sys::Function {
        self.start_shim()
    }
    fn type_(&self) -> ::wasm_bindgen::JsValue {
        self.type__shim()
    }
    fn write(&self) -> ::js_sys::Function {
        self.write_shim()
    }
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
        self.set_abort_shim(val);
        self
    }
    #[doc = "Change the `close` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSink`*"]
    pub fn close(&mut self, val: &::js_sys::Function) -> &mut Self {
        self.set_close_shim(val);
        self
    }
    #[doc = "Change the `start` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSink`*"]
    pub fn start(&mut self, val: &::js_sys::Function) -> &mut Self {
        self.set_start_shim(val);
        self
    }
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSink`*"]
    pub fn type_(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_type__shim(val);
        self
    }
    #[doc = "Change the `write` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSink`*"]
    pub fn write(&mut self, val: &::js_sys::Function) -> &mut Self {
        self.set_write_shim(val);
        self
    }
}
impl Default for UnderlyingSink {
    fn default() -> Self {
        Self::new()
    }
}
