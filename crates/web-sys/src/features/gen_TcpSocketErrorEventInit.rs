#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = TCPSocketErrorEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TcpSocketErrorEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocketErrorEventInit`*"]
    pub type TcpSocketErrorEventInit;
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &TcpSocketErrorEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &TcpSocketErrorEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &TcpSocketErrorEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &TcpSocketErrorEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &TcpSocketErrorEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &TcpSocketErrorEventInit, val: bool);
    #[wasm_bindgen(method, getter = "message")]
    fn message_shim(this: &TcpSocketErrorEventInit) -> String;
    #[wasm_bindgen(method, setter = "message")]
    fn set_message_shim(this: &TcpSocketErrorEventInit, val: &str);
    #[wasm_bindgen(method, getter = "name")]
    fn name_shim(this: &TcpSocketErrorEventInit) -> String;
    #[wasm_bindgen(method, setter = "name")]
    fn set_name_shim(this: &TcpSocketErrorEventInit, val: &str);
}
#[doc = "The trait to access properties on the `TcpSocketErrorEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `TcpSocketErrorEventInit`*"]
pub trait TcpSocketErrorEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocketErrorEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocketErrorEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocketErrorEventInit`*"]
    fn composed(&self) -> bool;
    #[doc = "Get the `message` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocketErrorEventInit`*"]
    fn message(&self) -> String;
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocketErrorEventInit`*"]
    fn name(&self) -> String;
}
impl TcpSocketErrorEventInitGetters for TcpSocketErrorEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    fn message(&self) -> String {
        self.message_shim()
    }
    fn name(&self) -> String {
        self.name_shim()
    }
}
impl TcpSocketErrorEventInit {
    #[doc = "Construct a new `TcpSocketErrorEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocketErrorEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocketErrorEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocketErrorEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocketErrorEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[doc = "Change the `message` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocketErrorEventInit`*"]
    pub fn message(&mut self, val: &str) -> &mut Self {
        self.set_message_shim(val);
        self
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocketErrorEventInit`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name_shim(val);
        self
    }
}
impl Default for TcpSocketErrorEventInit {
    fn default() -> Self {
        Self::new()
    }
}
