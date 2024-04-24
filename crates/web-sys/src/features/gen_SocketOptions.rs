#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = SocketOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SocketOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketOptions`*"]
    pub type SocketOptions;
    #[cfg(feature = "TcpSocketBinaryType")]
    #[wasm_bindgen(method, getter = "binaryType")]
    fn binary_type_shim(this: &SocketOptions) -> TcpSocketBinaryType;
    #[cfg(feature = "TcpSocketBinaryType")]
    #[wasm_bindgen(method, setter = "binaryType")]
    fn set_binary_type_shim(this: &SocketOptions, val: TcpSocketBinaryType);
    #[wasm_bindgen(method, getter = "useSecureTransport")]
    fn use_secure_transport_shim(this: &SocketOptions) -> bool;
    #[wasm_bindgen(method, setter = "useSecureTransport")]
    fn set_use_secure_transport_shim(this: &SocketOptions, val: bool);
}
#[doc = "The trait to access properties on the `SocketOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `SocketOptions`*"]
pub trait SocketOptionsGetters {
    #[cfg(feature = "TcpSocketBinaryType")]
    #[doc = "Get the `binaryType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketOptions`, `TcpSocketBinaryType`*"]
    fn binary_type(&self) -> TcpSocketBinaryType;
    #[doc = "Get the `useSecureTransport` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketOptions`*"]
    fn use_secure_transport(&self) -> bool;
}
impl SocketOptionsGetters for SocketOptions {
    #[cfg(feature = "TcpSocketBinaryType")]
    fn binary_type(&self) -> TcpSocketBinaryType {
        self.binary_type_shim()
    }
    fn use_secure_transport(&self) -> bool {
        self.use_secure_transport_shim()
    }
}
impl SocketOptions {
    #[doc = "Construct a new `SocketOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "TcpSocketBinaryType")]
    #[doc = "Change the `binaryType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketOptions`, `TcpSocketBinaryType`*"]
    pub fn binary_type(&mut self, val: TcpSocketBinaryType) -> &mut Self {
        self.set_binary_type_shim(val);
        self
    }
    #[doc = "Change the `useSecureTransport` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketOptions`*"]
    pub fn use_secure_transport(&mut self, val: bool) -> &mut Self {
        self.set_use_secure_transport_shim(val);
        self
    }
}
impl Default for SocketOptions {
    fn default() -> Self {
        Self::new()
    }
}
