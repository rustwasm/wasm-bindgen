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
    #[doc = "Get the `headers` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResponseInit`*"]
    #[wasm_bindgen(method, getter = "headers")]
    pub fn get_headers(this: &ResponseInit) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "headers")]
    fn set_headers(this: &ResponseInit, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `status` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResponseInit`*"]
    #[wasm_bindgen(method, getter = "status")]
    pub fn get_status(this: &ResponseInit) -> Option<u16>;
    #[wasm_bindgen(method, setter = "status")]
    fn set_status(this: &ResponseInit, val: u16);
    #[doc = "Get the `statusText` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResponseInit`*"]
    #[wasm_bindgen(method, getter = "statusText")]
    pub fn get_status_text(this: &ResponseInit) -> Option<String>;
    #[wasm_bindgen(method, setter = "statusText")]
    fn set_status_text(this: &ResponseInit, val: &str);
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
        self.set_headers(val);
        self
    }
    #[doc = "Change the `status` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResponseInit`*"]
    pub fn status(&mut self, val: u16) -> &mut Self {
        self.set_status(val);
        self
    }
    #[doc = "Change the `statusText` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResponseInit`*"]
    pub fn status_text(&mut self, val: &str) -> &mut Self {
        self.set_status_text(val);
        self
    }
}
impl Default for ResponseInit {
    fn default() -> Self {
        Self::new()
    }
}
