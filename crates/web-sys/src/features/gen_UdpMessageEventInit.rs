#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = UDPMessageEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UdpMessageEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpMessageEventInit`*"]
    pub type UdpMessageEventInit;
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &UdpMessageEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &UdpMessageEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &UdpMessageEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &UdpMessageEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &UdpMessageEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &UdpMessageEventInit, val: bool);
    #[wasm_bindgen(method, getter = "data")]
    fn data_shim(this: &UdpMessageEventInit) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "data")]
    fn set_data_shim(this: &UdpMessageEventInit, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "remoteAddress")]
    fn remote_address_shim(this: &UdpMessageEventInit) -> String;
    #[wasm_bindgen(method, setter = "remoteAddress")]
    fn set_remote_address_shim(this: &UdpMessageEventInit, val: &str);
    #[wasm_bindgen(method, getter = "remotePort")]
    fn remote_port_shim(this: &UdpMessageEventInit) -> u16;
    #[wasm_bindgen(method, setter = "remotePort")]
    fn set_remote_port_shim(this: &UdpMessageEventInit, val: u16);
}
#[doc = "The trait to access properties on the `UdpMessageEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `UdpMessageEventInit`*"]
pub trait UdpMessageEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpMessageEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpMessageEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpMessageEventInit`*"]
    fn composed(&self) -> bool;
    #[doc = "Get the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpMessageEventInit`*"]
    fn data(&self) -> ::wasm_bindgen::JsValue;
    #[doc = "Get the `remoteAddress` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpMessageEventInit`*"]
    fn remote_address(&self) -> String;
    #[doc = "Get the `remotePort` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpMessageEventInit`*"]
    fn remote_port(&self) -> u16;
}
impl UdpMessageEventInitGetters for UdpMessageEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    fn data(&self) -> ::wasm_bindgen::JsValue {
        self.data_shim()
    }
    fn remote_address(&self) -> String {
        self.remote_address_shim()
    }
    fn remote_port(&self) -> u16 {
        self.remote_port_shim()
    }
}
impl UdpMessageEventInit {
    #[doc = "Construct a new `UdpMessageEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpMessageEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpMessageEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpMessageEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpMessageEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[doc = "Change the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpMessageEventInit`*"]
    pub fn data(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_data_shim(val);
        self
    }
    #[doc = "Change the `remoteAddress` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpMessageEventInit`*"]
    pub fn remote_address(&mut self, val: &str) -> &mut Self {
        self.set_remote_address_shim(val);
        self
    }
    #[doc = "Change the `remotePort` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpMessageEventInit`*"]
    pub fn remote_port(&mut self, val: u16) -> &mut Self {
        self.set_remote_port_shim(val);
        self
    }
}
impl Default for UdpMessageEventInit {
    fn default() -> Self {
        Self::new()
    }
}
