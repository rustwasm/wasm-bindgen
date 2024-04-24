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
    #[wasm_bindgen(method, getter = "active")]
    fn active_shim(this: &HttpConnectionElement) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "active")]
    fn set_active_shim(this: &HttpConnectionElement, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "halfOpens")]
    fn half_opens_shim(this: &HttpConnectionElement) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "halfOpens")]
    fn set_half_opens_shim(this: &HttpConnectionElement, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "host")]
    fn host_shim(this: &HttpConnectionElement) -> String;
    #[wasm_bindgen(method, setter = "host")]
    fn set_host_shim(this: &HttpConnectionElement, val: &str);
    #[wasm_bindgen(method, getter = "idle")]
    fn idle_shim(this: &HttpConnectionElement) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "idle")]
    fn set_idle_shim(this: &HttpConnectionElement, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "port")]
    fn port_shim(this: &HttpConnectionElement) -> u32;
    #[wasm_bindgen(method, setter = "port")]
    fn set_port_shim(this: &HttpConnectionElement, val: u32);
    #[wasm_bindgen(method, getter = "spdy")]
    fn spdy_shim(this: &HttpConnectionElement) -> bool;
    #[wasm_bindgen(method, setter = "spdy")]
    fn set_spdy_shim(this: &HttpConnectionElement, val: bool);
    #[wasm_bindgen(method, getter = "ssl")]
    fn ssl_shim(this: &HttpConnectionElement) -> bool;
    #[wasm_bindgen(method, setter = "ssl")]
    fn set_ssl_shim(this: &HttpConnectionElement, val: bool);
}
#[doc = "The trait to access properties on the `HttpConnectionElement` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
pub trait HttpConnectionElementGetters {
    #[doc = "Get the `active` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
    fn active(&self) -> ::js_sys::Array;
    #[doc = "Get the `halfOpens` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
    fn half_opens(&self) -> ::js_sys::Array;
    #[doc = "Get the `host` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
    fn host(&self) -> String;
    #[doc = "Get the `idle` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
    fn idle(&self) -> ::js_sys::Array;
    #[doc = "Get the `port` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
    fn port(&self) -> u32;
    #[doc = "Get the `spdy` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
    fn spdy(&self) -> bool;
    #[doc = "Get the `ssl` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
    fn ssl(&self) -> bool;
}
impl HttpConnectionElementGetters for HttpConnectionElement {
    fn active(&self) -> ::js_sys::Array {
        self.active_shim()
    }
    fn half_opens(&self) -> ::js_sys::Array {
        self.half_opens_shim()
    }
    fn host(&self) -> String {
        self.host_shim()
    }
    fn idle(&self) -> ::js_sys::Array {
        self.idle_shim()
    }
    fn port(&self) -> u32 {
        self.port_shim()
    }
    fn spdy(&self) -> bool {
        self.spdy_shim()
    }
    fn ssl(&self) -> bool {
        self.ssl_shim()
    }
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
        self.set_active_shim(val);
        self
    }
    #[doc = "Change the `halfOpens` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
    pub fn half_opens(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_half_opens_shim(val);
        self
    }
    #[doc = "Change the `host` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
    pub fn host(&mut self, val: &str) -> &mut Self {
        self.set_host_shim(val);
        self
    }
    #[doc = "Change the `idle` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
    pub fn idle(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_idle_shim(val);
        self
    }
    #[doc = "Change the `port` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
    pub fn port(&mut self, val: u32) -> &mut Self {
        self.set_port_shim(val);
        self
    }
    #[doc = "Change the `spdy` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
    pub fn spdy(&mut self, val: bool) -> &mut Self {
        self.set_spdy_shim(val);
        self
    }
    #[doc = "Change the `ssl` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
    pub fn ssl(&mut self, val: bool) -> &mut Self {
        self.set_ssl_shim(val);
        self
    }
}
impl Default for HttpConnectionElement {
    fn default() -> Self {
        Self::new()
    }
}
