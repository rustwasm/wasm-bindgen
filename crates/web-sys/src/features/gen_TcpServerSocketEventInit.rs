#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = TCPServerSocketEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TcpServerSocketEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpServerSocketEventInit`*"]
    pub type TcpServerSocketEventInit;
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &TcpServerSocketEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &TcpServerSocketEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &TcpServerSocketEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &TcpServerSocketEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &TcpServerSocketEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &TcpServerSocketEventInit, val: bool);
    #[cfg(feature = "TcpSocket")]
    #[wasm_bindgen(method, getter = "socket")]
    fn socket_shim(this: &TcpServerSocketEventInit) -> Option<TcpSocket>;
    #[cfg(feature = "TcpSocket")]
    #[wasm_bindgen(method, setter = "socket")]
    fn set_socket_shim(this: &TcpServerSocketEventInit, val: Option<&TcpSocket>);
}
#[doc = "The trait to access properties on the `TcpServerSocketEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `TcpServerSocketEventInit`*"]
pub trait TcpServerSocketEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpServerSocketEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpServerSocketEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpServerSocketEventInit`*"]
    fn composed(&self) -> bool;
    #[cfg(feature = "TcpSocket")]
    #[doc = "Get the `socket` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpServerSocketEventInit`, `TcpSocket`*"]
    fn socket(&self) -> Option<TcpSocket>;
}
impl TcpServerSocketEventInitGetters for TcpServerSocketEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    #[cfg(feature = "TcpSocket")]
    fn socket(&self) -> Option<TcpSocket> {
        self.socket_shim()
    }
}
impl TcpServerSocketEventInit {
    #[doc = "Construct a new `TcpServerSocketEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpServerSocketEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpServerSocketEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpServerSocketEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpServerSocketEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[cfg(feature = "TcpSocket")]
    #[doc = "Change the `socket` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpServerSocketEventInit`, `TcpSocket`*"]
    pub fn socket(&mut self, val: Option<&TcpSocket>) -> &mut Self {
        self.set_socket_shim(val);
        self
    }
}
impl Default for TcpServerSocketEventInit {
    fn default() -> Self {
        Self::new()
    }
}
