#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = HttpConnectionElement)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HttpConnectionElement` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
    pub type HttpConnectionElement;
    #[wasm_bindgen(method, setter = "active")]
    fn active_shim(this: &HttpConnectionElement, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "halfOpens")]
    fn half_opens_shim(this: &HttpConnectionElement, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "host")]
    fn host_shim(this: &HttpConnectionElement, val: &str);
    #[wasm_bindgen(method, setter = "idle")]
    fn idle_shim(this: &HttpConnectionElement, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "port")]
    fn port_shim(this: &HttpConnectionElement, val: u32);
    #[wasm_bindgen(method, setter = "spdy")]
    fn spdy_shim(this: &HttpConnectionElement, val: bool);
    #[wasm_bindgen(method, setter = "ssl")]
    fn ssl_shim(this: &HttpConnectionElement, val: bool);
}
impl HttpConnectionElement {
    #[doc = "Construct a new `HttpConnectionElement`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `active` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
    pub fn active(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.active_shim(val);
        self
    }
    #[doc = "Change the `halfOpens` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
    pub fn half_opens(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.half_opens_shim(val);
        self
    }
    #[doc = "Change the `host` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
    pub fn host(&mut self, val: &str) -> &mut Self {
        self.host_shim(val);
        self
    }
    #[doc = "Change the `idle` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
    pub fn idle(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.idle_shim(val);
        self
    }
    #[doc = "Change the `port` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
    pub fn port(&mut self, val: u32) -> &mut Self {
        self.port_shim(val);
        self
    }
    #[doc = "Change the `spdy` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
    pub fn spdy(&mut self, val: bool) -> &mut Self {
        self.spdy_shim(val);
        self
    }
    #[doc = "Change the `ssl` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
    pub fn ssl(&mut self, val: bool) -> &mut Self {
        self.ssl_shim(val);
        self
    }
}
impl Default for HttpConnectionElement {
    fn default() -> Self {
        Self::new()
    }
}
