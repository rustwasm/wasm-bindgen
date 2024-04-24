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
    #[wasm_bindgen(method, getter = "active")]
    fn active_shim(this: &SocketElement) -> bool;
    #[wasm_bindgen(method, setter = "active")]
    fn set_active_shim(this: &SocketElement, val: bool);
    #[wasm_bindgen(method, getter = "host")]
    fn host_shim(this: &SocketElement) -> String;
    #[wasm_bindgen(method, setter = "host")]
    fn set_host_shim(this: &SocketElement, val: &str);
    #[wasm_bindgen(method, getter = "port")]
    fn port_shim(this: &SocketElement) -> u32;
    #[wasm_bindgen(method, setter = "port")]
    fn set_port_shim(this: &SocketElement, val: u32);
    #[wasm_bindgen(method, getter = "received")]
    fn received_shim(this: &SocketElement) -> f64;
    #[wasm_bindgen(method, setter = "received")]
    fn set_received_shim(this: &SocketElement, val: f64);
    #[wasm_bindgen(method, getter = "sent")]
    fn sent_shim(this: &SocketElement) -> f64;
    #[wasm_bindgen(method, setter = "sent")]
    fn set_sent_shim(this: &SocketElement, val: f64);
    #[wasm_bindgen(method, getter = "tcp")]
    fn tcp_shim(this: &SocketElement) -> bool;
    #[wasm_bindgen(method, setter = "tcp")]
    fn set_tcp_shim(this: &SocketElement, val: bool);
}
#[doc = "The trait to access properties on the `SocketElement` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
pub trait SocketElementGetters {
    #[doc = "Get the `active` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
    fn active(&self) -> bool;
    #[doc = "Get the `host` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
    fn host(&self) -> String;
    #[doc = "Get the `port` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
    fn port(&self) -> u32;
    #[doc = "Get the `received` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
    fn received(&self) -> f64;
    #[doc = "Get the `sent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
    fn sent(&self) -> f64;
    #[doc = "Get the `tcp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
    fn tcp(&self) -> bool;
}
impl SocketElementGetters for SocketElement {
    fn active(&self) -> bool {
        self.active_shim()
    }
    fn host(&self) -> String {
        self.host_shim()
    }
    fn port(&self) -> u32 {
        self.port_shim()
    }
    fn received(&self) -> f64 {
        self.received_shim()
    }
    fn sent(&self) -> f64 {
        self.sent_shim()
    }
    fn tcp(&self) -> bool {
        self.tcp_shim()
    }
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
        self.set_active_shim(val);
        self
    }
    #[doc = "Change the `host` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
    pub fn host(&mut self, val: &str) -> &mut Self {
        self.set_host_shim(val);
        self
    }
    #[doc = "Change the `port` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
    pub fn port(&mut self, val: u32) -> &mut Self {
        self.set_port_shim(val);
        self
    }
    #[doc = "Change the `received` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
    pub fn received(&mut self, val: f64) -> &mut Self {
        self.set_received_shim(val);
        self
    }
    #[doc = "Change the `sent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
    pub fn sent(&mut self, val: f64) -> &mut Self {
        self.set_sent_shim(val);
        self
    }
    #[doc = "Change the `tcp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
    pub fn tcp(&mut self, val: bool) -> &mut Self {
        self.set_tcp_shim(val);
        self
    }
}
impl Default for SocketElement {
    fn default() -> Self {
        Self::new()
    }
}
