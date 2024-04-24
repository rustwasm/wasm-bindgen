#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCIdentityProviderOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcIdentityProviderOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProviderOptions`*"]
    pub type RtcIdentityProviderOptions;
    #[wasm_bindgen(method, getter = "peerIdentity")]
    fn peer_identity_shim(this: &RtcIdentityProviderOptions) -> &str;
    #[wasm_bindgen(method, setter = "peerIdentity")]
    fn set_peer_identity_shim(this: &RtcIdentityProviderOptions, val: &str);
    #[wasm_bindgen(method, getter = "protocol")]
    fn protocol_shim(this: &RtcIdentityProviderOptions) -> &str;
    #[wasm_bindgen(method, setter = "protocol")]
    fn set_protocol_shim(this: &RtcIdentityProviderOptions, val: &str);
    #[wasm_bindgen(method, getter = "usernameHint")]
    fn username_hint_shim(this: &RtcIdentityProviderOptions) -> &str;
    #[wasm_bindgen(method, setter = "usernameHint")]
    fn set_username_hint_shim(this: &RtcIdentityProviderOptions, val: &str);
}
#[doc = "The trait to access properties on the `RtcIdentityProviderOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcIdentityProviderOptions`*"]
pub trait RtcIdentityProviderOptionsGetters {
    #[doc = "Get the `peerIdentity` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProviderOptions`*"]
    fn peer_identity(&self) -> &str;
    #[doc = "Get the `protocol` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProviderOptions`*"]
    fn protocol(&self) -> &str;
    #[doc = "Get the `usernameHint` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProviderOptions`*"]
    fn username_hint(&self) -> &str;
}
impl RtcIdentityProviderOptionsGetters for RtcIdentityProviderOptions {
    fn peer_identity(&self) -> &str {
        self.peer_identity_shim()
    }
    fn protocol(&self) -> &str {
        self.protocol_shim()
    }
    fn username_hint(&self) -> &str {
        self.username_hint_shim()
    }
}
impl RtcIdentityProviderOptions {
    #[doc = "Construct a new `RtcIdentityProviderOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProviderOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `peerIdentity` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProviderOptions`*"]
    pub fn peer_identity(&mut self, val: &str) -> &mut Self {
        self.set_peer_identity_shim(val);
        self
    }
    #[doc = "Change the `protocol` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProviderOptions`*"]
    pub fn protocol(&mut self, val: &str) -> &mut Self {
        self.set_protocol_shim(val);
        self
    }
    #[doc = "Change the `usernameHint` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProviderOptions`*"]
    pub fn username_hint(&mut self, val: &str) -> &mut Self {
        self.set_username_hint_shim(val);
        self
    }
}
impl Default for RtcIdentityProviderOptions {
    fn default() -> Self {
        Self::new()
    }
}
