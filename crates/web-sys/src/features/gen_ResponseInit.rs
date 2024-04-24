#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ResponseInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ResponseInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResponseInit`*"]
    pub type ResponseInit;
    #[wasm_bindgen(method, getter = "headers")]
    fn headers_shim(this: &ResponseInit) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "headers")]
    fn set_headers_shim(this: &ResponseInit, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "status")]
    fn status_shim(this: &ResponseInit) -> u16;
    #[wasm_bindgen(method, setter = "status")]
    fn set_status_shim(this: &ResponseInit, val: u16);
    #[wasm_bindgen(method, getter = "statusText")]
    fn status_text_shim(this: &ResponseInit) -> &str;
    #[wasm_bindgen(method, setter = "statusText")]
    fn set_status_text_shim(this: &ResponseInit, val: &str);
}
#[doc = "The trait to access properties on the `ResponseInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ResponseInit`*"]
pub trait ResponseInitGetters {
    #[doc = "Get the `headers` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResponseInit`*"]
    fn headers(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `status` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResponseInit`*"]
    fn status(&self) -> u16;
    #[doc = "Get the `statusText` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResponseInit`*"]
    fn status_text(&self) -> &str;
}
impl ResponseInitGetters for ResponseInit {
    fn headers(&self) -> &::wasm_bindgen::JsValue {
        self.headers_shim()
    }
    fn status(&self) -> u16 {
        self.status_shim()
    }
    fn status_text(&self) -> &str {
        self.status_text_shim()
    }
}
impl ResponseInit {
    #[doc = "Construct a new `ResponseInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResponseInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `headers` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResponseInit`*"]
    pub fn headers(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_headers_shim(val);
        self
    }
    #[doc = "Change the `status` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResponseInit`*"]
    pub fn status(&mut self, val: u16) -> &mut Self {
        self.set_status_shim(val);
        self
    }
    #[doc = "Change the `statusText` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResponseInit`*"]
    pub fn status_text(&mut self, val: &str) -> &mut Self {
        self.set_status_text_shim(val);
        self
    }
}
impl Default for ResponseInit {
    fn default() -> Self {
        Self::new()
    }
}
