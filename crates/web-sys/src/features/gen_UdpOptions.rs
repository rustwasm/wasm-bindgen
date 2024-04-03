#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = UDPOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UdpOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
    pub type UdpOptions;
    #[wasm_bindgen(method, setter = "addressReuse")]
    fn address_reuse_shim(this: &UdpOptions, val: bool);
    #[wasm_bindgen(method, setter = "localAddress")]
    fn local_address_shim(this: &UdpOptions, val: &str);
    #[wasm_bindgen(method, setter = "localPort")]
    fn local_port_shim(this: &UdpOptions, val: u16);
    #[wasm_bindgen(method, setter = "loopback")]
    fn loopback_shim(this: &UdpOptions, val: bool);
    #[wasm_bindgen(method, setter = "remoteAddress")]
    fn remote_address_shim(this: &UdpOptions, val: &str);
    #[wasm_bindgen(method, setter = "remotePort")]
    fn remote_port_shim(this: &UdpOptions, val: u16);
}
impl UdpOptions {
    #[doc = "Construct a new `UdpOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `addressReuse` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
    pub fn address_reuse(&mut self, val: bool) -> &mut Self {
        self.address_reuse_shim(val);
        self
    }
    #[doc = "Change the `localAddress` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
    pub fn local_address(&mut self, val: &str) -> &mut Self {
        self.local_address_shim(val);
        self
    }
    #[doc = "Change the `localPort` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
    pub fn local_port(&mut self, val: u16) -> &mut Self {
        self.local_port_shim(val);
        self
    }
    #[doc = "Change the `loopback` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
    pub fn loopback(&mut self, val: bool) -> &mut Self {
        self.loopback_shim(val);
        self
    }
    #[doc = "Change the `remoteAddress` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
    pub fn remote_address(&mut self, val: &str) -> &mut Self {
        self.remote_address_shim(val);
        self
    }
    #[doc = "Change the `remotePort` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
    pub fn remote_port(&mut self, val: u16) -> &mut Self {
        self.remote_port_shim(val);
        self
    }
}
impl Default for UdpOptions {
    fn default() -> Self {
        Self::new()
    }
}
