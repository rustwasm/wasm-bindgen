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
    #[wasm_bindgen(method, setter = "active")]
    fn active_shim(this: &SocketElement, val: bool);
    #[wasm_bindgen(method, setter = "host")]
    fn host_shim(this: &SocketElement, val: &str);
    #[wasm_bindgen(method, setter = "port")]
    fn port_shim(this: &SocketElement, val: u32);
    #[wasm_bindgen(method, setter = "received")]
    fn received_shim(this: &SocketElement, val: f64);
    #[wasm_bindgen(method, setter = "sent")]
    fn sent_shim(this: &SocketElement, val: f64);
    #[wasm_bindgen(method, setter = "tcp")]
    fn tcp_shim(this: &SocketElement, val: bool);
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
        self.active_shim(val);
        self
    }
    #[doc = "Change the `host` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
    pub fn host(&mut self, val: &str) -> &mut Self {
        self.host_shim(val);
        self
    }
    #[doc = "Change the `port` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
    pub fn port(&mut self, val: u32) -> &mut Self {
        self.port_shim(val);
        self
    }
    #[doc = "Change the `received` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
    pub fn received(&mut self, val: f64) -> &mut Self {
        self.received_shim(val);
        self
    }
    #[doc = "Change the `sent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
    pub fn sent(&mut self, val: f64) -> &mut Self {
        self.sent_shim(val);
        self
    }
    #[doc = "Change the `tcp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
    pub fn tcp(&mut self, val: bool) -> &mut Self {
        self.tcp_shim(val);
        self
    }
}
impl Default for SocketElement {
    fn default() -> Self {
        Self::new()
    }
}
