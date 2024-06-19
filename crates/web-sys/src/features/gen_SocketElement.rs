#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = SocketElement)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SocketElement` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
    pub type SocketElement;
    #[doc = "Get the `active` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
    #[wasm_bindgen(method, getter = "active")]
    pub fn get_active(this: &SocketElement) -> Option<bool>;
    #[wasm_bindgen(method, setter = "active")]
    fn set_active(this: &SocketElement, val: bool);
    #[doc = "Get the `host` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
    #[wasm_bindgen(method, getter = "host")]
    pub fn get_host(this: &SocketElement) -> Option<String>;
    #[wasm_bindgen(method, setter = "host")]
    fn set_host(this: &SocketElement, val: &str);
    #[doc = "Get the `port` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
    #[wasm_bindgen(method, getter = "port")]
    pub fn get_port(this: &SocketElement) -> Option<u32>;
    #[wasm_bindgen(method, setter = "port")]
    fn set_port(this: &SocketElement, val: u32);
    #[doc = "Get the `received` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
    #[wasm_bindgen(method, getter = "received")]
    pub fn get_received(this: &SocketElement) -> Option<f64>;
    #[wasm_bindgen(method, setter = "received")]
    fn set_received(this: &SocketElement, val: f64);
    #[doc = "Get the `sent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
    #[wasm_bindgen(method, getter = "sent")]
    pub fn get_sent(this: &SocketElement) -> Option<f64>;
    #[wasm_bindgen(method, setter = "sent")]
    fn set_sent(this: &SocketElement, val: f64);
    #[doc = "Get the `tcp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
    #[wasm_bindgen(method, getter = "tcp")]
    pub fn get_tcp(this: &SocketElement) -> Option<bool>;
    #[wasm_bindgen(method, setter = "tcp")]
    fn set_tcp(this: &SocketElement, val: bool);
}
impl SocketElement {
    #[doc = "Construct a new `SocketElement`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `active` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
    pub fn active(&mut self, val: bool) -> &mut Self {
        self.set_active(val);
        self
    }
    #[doc = "Change the `host` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
    pub fn host(&mut self, val: &str) -> &mut Self {
        self.set_host(val);
        self
    }
    #[doc = "Change the `port` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
    pub fn port(&mut self, val: u32) -> &mut Self {
        self.set_port(val);
        self
    }
    #[doc = "Change the `received` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
    pub fn received(&mut self, val: f64) -> &mut Self {
        self.set_received(val);
        self
    }
    #[doc = "Change the `sent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
    pub fn sent(&mut self, val: f64) -> &mut Self {
        self.set_sent(val);
        self
    }
    #[doc = "Change the `tcp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
    pub fn tcp(&mut self, val: bool) -> &mut Self {
        self.set_tcp(val);
        self
    }
}
impl Default for SocketElement {
    fn default() -> Self {
        Self::new()
    }
}
