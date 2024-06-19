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
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpMessageEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &UdpMessageEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles(this: &UdpMessageEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpMessageEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &UdpMessageEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable(this: &UdpMessageEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpMessageEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &UdpMessageEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed(this: &UdpMessageEventInit, val: bool);
    #[doc = "Get the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpMessageEventInit`*"]
    #[wasm_bindgen(method, getter = "data")]
    pub fn get_data(this: &UdpMessageEventInit) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "data")]
    fn set_data(this: &UdpMessageEventInit, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `remoteAddress` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpMessageEventInit`*"]
    #[wasm_bindgen(method, getter = "remoteAddress")]
    pub fn get_remote_address(this: &UdpMessageEventInit) -> Option<String>;
    #[wasm_bindgen(method, setter = "remoteAddress")]
    fn set_remote_address(this: &UdpMessageEventInit, val: &str);
    #[doc = "Get the `remotePort` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpMessageEventInit`*"]
    #[wasm_bindgen(method, getter = "remotePort")]
    pub fn get_remote_port(this: &UdpMessageEventInit) -> Option<u16>;
    #[wasm_bindgen(method, setter = "remotePort")]
    fn set_remote_port(this: &UdpMessageEventInit, val: u16);
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
        self.set_bubbles(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpMessageEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpMessageEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed(val);
        self
    }
    #[doc = "Change the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpMessageEventInit`*"]
    pub fn data(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_data(val);
        self
    }
    #[doc = "Change the `remoteAddress` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpMessageEventInit`*"]
    pub fn remote_address(&mut self, val: &str) -> &mut Self {
        self.set_remote_address(val);
        self
    }
    #[doc = "Change the `remotePort` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpMessageEventInit`*"]
    pub fn remote_port(&mut self, val: u16) -> &mut Self {
        self.set_remote_port(val);
        self
    }
}
impl Default for UdpMessageEventInit {
    fn default() -> Self {
        Self::new()
    }
}
