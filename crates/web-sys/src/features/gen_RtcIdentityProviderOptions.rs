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
    #[wasm_bindgen(method, setter = "peerIdentity")]
    fn peer_identity_shim(this: &RtcIdentityProviderOptions, val: &str);
    #[wasm_bindgen(method, setter = "protocol")]
    fn protocol_shim(this: &RtcIdentityProviderOptions, val: &str);
    #[wasm_bindgen(method, setter = "usernameHint")]
    fn username_hint_shim(this: &RtcIdentityProviderOptions, val: &str);
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
        self.peer_identity_shim(val);
        self
    }
    #[doc = "Change the `protocol` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProviderOptions`*"]
    pub fn protocol(&mut self, val: &str) -> &mut Self {
        self.protocol_shim(val);
        self
    }
    #[doc = "Change the `usernameHint` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProviderOptions`*"]
    pub fn username_hint(&mut self, val: &str) -> &mut Self {
        self.username_hint_shim(val);
        self
    }
}
impl Default for RtcIdentityProviderOptions {
    fn default() -> Self {
        Self::new()
    }
}
