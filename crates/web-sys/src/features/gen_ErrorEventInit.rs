#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ErrorEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ErrorEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
    pub type ErrorEventInit;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn bubbles_shim(this: &ErrorEventInit, val: bool);
    #[wasm_bindgen(method, setter = "cancelable")]
    fn cancelable_shim(this: &ErrorEventInit, val: bool);
    #[wasm_bindgen(method, setter = "composed")]
    fn composed_shim(this: &ErrorEventInit, val: bool);
    #[wasm_bindgen(method, setter = "colno")]
    fn colno_shim(this: &ErrorEventInit, val: u32);
    #[wasm_bindgen(method, setter = "error")]
    fn error_shim(this: &ErrorEventInit, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "filename")]
    fn filename_shim(this: &ErrorEventInit, val: &str);
    #[wasm_bindgen(method, setter = "lineno")]
    fn lineno_shim(this: &ErrorEventInit, val: u32);
    #[wasm_bindgen(method, setter = "message")]
    fn message_shim(this: &ErrorEventInit, val: &str);
}
impl ErrorEventInit {
    #[doc = "Construct a new `ErrorEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.composed_shim(val);
        self
    }
    #[doc = "Change the `colno` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
    pub fn colno(&mut self, val: u32) -> &mut Self {
        self.colno_shim(val);
        self
    }
    #[doc = "Change the `error` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
    pub fn error(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.error_shim(val);
        self
    }
    #[doc = "Change the `filename` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
    pub fn filename(&mut self, val: &str) -> &mut Self {
        self.filename_shim(val);
        self
    }
    #[doc = "Change the `lineno` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
    pub fn lineno(&mut self, val: u32) -> &mut Self {
        self.lineno_shim(val);
        self
    }
    #[doc = "Change the `message` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorEventInit`*"]
    pub fn message(&mut self, val: &str) -> &mut Self {
        self.message_shim(val);
        self
    }
}
impl Default for ErrorEventInit {
    fn default() -> Self {
        Self::new()
    }
}
