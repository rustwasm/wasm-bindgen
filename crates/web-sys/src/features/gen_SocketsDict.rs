#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = SocketsDict)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SocketsDict` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketsDict`*"]
    pub type SocketsDict;
    #[wasm_bindgen(method, getter = "received")]
    fn received_shim(this: &SocketsDict) -> f64;
    #[wasm_bindgen(method, setter = "received")]
    fn set_received_shim(this: &SocketsDict, val: f64);
    #[wasm_bindgen(method, getter = "sent")]
    fn sent_shim(this: &SocketsDict) -> f64;
    #[wasm_bindgen(method, setter = "sent")]
    fn set_sent_shim(this: &SocketsDict, val: f64);
    #[wasm_bindgen(method, getter = "sockets")]
    fn sockets_shim(this: &SocketsDict) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "sockets")]
    fn set_sockets_shim(this: &SocketsDict, val: &::wasm_bindgen::JsValue);
}
#[doc = "The trait to access properties on the `SocketsDict` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `SocketsDict`*"]
pub trait SocketsDictGetters {
    #[doc = "Get the `received` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketsDict`*"]
    fn received(&self) -> f64;
    #[doc = "Get the `sent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketsDict`*"]
    fn sent(&self) -> f64;
    #[doc = "Get the `sockets` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketsDict`*"]
    fn sockets(&self) -> ::js_sys::Array;
}
impl SocketsDictGetters for SocketsDict {
    fn received(&self) -> f64 {
        self.received_shim()
    }
    fn sent(&self) -> f64 {
        self.sent_shim()
    }
    fn sockets(&self) -> ::js_sys::Array {
        self.sockets_shim()
    }
}
impl SocketsDict {
    #[doc = "Construct a new `SocketsDict`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketsDict`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `received` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketsDict`*"]
    pub fn received(&mut self, val: f64) -> &mut Self {
        self.set_received_shim(val);
        self
    }
    #[doc = "Change the `sent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketsDict`*"]
    pub fn sent(&mut self, val: f64) -> &mut Self {
        self.set_sent_shim(val);
        self
    }
    #[doc = "Change the `sockets` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketsDict`*"]
    pub fn sockets(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_sockets_shim(val);
        self
    }
}
impl Default for SocketsDict {
    fn default() -> Self {
        Self::new()
    }
}
