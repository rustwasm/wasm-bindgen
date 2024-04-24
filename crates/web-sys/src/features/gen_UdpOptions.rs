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
    #[wasm_bindgen(method, getter = "addressReuse")]
    fn address_reuse_shim(this: &UdpOptions) -> bool;
    #[wasm_bindgen(method, setter = "addressReuse")]
    fn set_address_reuse_shim(this: &UdpOptions, val: bool);
    #[wasm_bindgen(method, getter = "localAddress")]
    fn local_address_shim(this: &UdpOptions) -> String;
    #[wasm_bindgen(method, setter = "localAddress")]
    fn set_local_address_shim(this: &UdpOptions, val: &str);
    #[wasm_bindgen(method, getter = "localPort")]
    fn local_port_shim(this: &UdpOptions) -> u16;
    #[wasm_bindgen(method, setter = "localPort")]
    fn set_local_port_shim(this: &UdpOptions, val: u16);
    #[wasm_bindgen(method, getter = "loopback")]
    fn loopback_shim(this: &UdpOptions) -> bool;
    #[wasm_bindgen(method, setter = "loopback")]
    fn set_loopback_shim(this: &UdpOptions, val: bool);
    #[wasm_bindgen(method, getter = "remoteAddress")]
    fn remote_address_shim(this: &UdpOptions) -> String;
    #[wasm_bindgen(method, setter = "remoteAddress")]
    fn set_remote_address_shim(this: &UdpOptions, val: &str);
    #[wasm_bindgen(method, getter = "remotePort")]
    fn remote_port_shim(this: &UdpOptions) -> u16;
    #[wasm_bindgen(method, setter = "remotePort")]
    fn set_remote_port_shim(this: &UdpOptions, val: u16);
}
#[doc = "The trait to access properties on the `UdpOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
pub trait UdpOptionsGetters {
    #[doc = "Get the `addressReuse` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
    fn address_reuse(&self) -> bool;
    #[doc = "Get the `localAddress` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
    fn local_address(&self) -> String;
    #[doc = "Get the `localPort` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
    fn local_port(&self) -> u16;
    #[doc = "Get the `loopback` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
    fn loopback(&self) -> bool;
    #[doc = "Get the `remoteAddress` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
    fn remote_address(&self) -> String;
    #[doc = "Get the `remotePort` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
    fn remote_port(&self) -> u16;
}
impl UdpOptionsGetters for UdpOptions {
    fn address_reuse(&self) -> bool {
        self.address_reuse_shim()
    }
    fn local_address(&self) -> String {
        self.local_address_shim()
    }
    fn local_port(&self) -> u16 {
        self.local_port_shim()
    }
    fn loopback(&self) -> bool {
        self.loopback_shim()
    }
    fn remote_address(&self) -> String {
        self.remote_address_shim()
    }
    fn remote_port(&self) -> u16 {
        self.remote_port_shim()
    }
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
        self.set_address_reuse_shim(val);
        self
    }
    #[doc = "Change the `localAddress` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
    pub fn local_address(&mut self, val: &str) -> &mut Self {
        self.set_local_address_shim(val);
        self
    }
    #[doc = "Change the `localPort` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
    pub fn local_port(&mut self, val: u16) -> &mut Self {
        self.set_local_port_shim(val);
        self
    }
    #[doc = "Change the `loopback` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
    pub fn loopback(&mut self, val: bool) -> &mut Self {
        self.set_loopback_shim(val);
        self
    }
    #[doc = "Change the `remoteAddress` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
    pub fn remote_address(&mut self, val: &str) -> &mut Self {
        self.set_remote_address_shim(val);
        self
    }
    #[doc = "Change the `remotePort` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
    pub fn remote_port(&mut self, val: u16) -> &mut Self {
        self.set_remote_port_shim(val);
        self
    }
}
impl Default for UdpOptions {
    fn default() -> Self {
        Self::new()
    }
}
