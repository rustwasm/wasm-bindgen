#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCConfiguration)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcConfiguration` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcConfiguration`*"]
    pub type RtcConfiguration;
    #[cfg(feature = "RtcBundlePolicy")]
    #[wasm_bindgen(method, setter = "bundlePolicy")]
    fn bundle_policy_shim(this: &RtcConfiguration, val: RtcBundlePolicy);
    #[wasm_bindgen(method, setter = "certificates")]
    fn certificates_shim(this: &RtcConfiguration, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter = "iceServers")]
    fn ice_servers_shim(this: &RtcConfiguration, val: &::wasm_bindgen::JsValue);
    #[cfg(feature = "RtcIceTransportPolicy")]
    #[wasm_bindgen(method, setter = "iceTransportPolicy")]
    fn ice_transport_policy_shim(this: &RtcConfiguration, val: RtcIceTransportPolicy);
    #[wasm_bindgen(method, setter = "peerIdentity")]
    fn peer_identity_shim(this: &RtcConfiguration, val: Option<&str>);
}
impl RtcConfiguration {
    #[doc = "Construct a new `RtcConfiguration`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcConfiguration`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "RtcBundlePolicy")]
    #[doc = "Change the `bundlePolicy` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcBundlePolicy`, `RtcConfiguration`*"]
    pub fn bundle_policy(&mut self, val: RtcBundlePolicy) -> &mut Self {
        self.bundle_policy_shim(val);
        self
    }
    #[doc = "Change the `certificates` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcConfiguration`*"]
    pub fn certificates(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.certificates_shim(val);
        self
    }
    #[doc = "Change the `iceServers` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcConfiguration`*"]
    pub fn ice_servers(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.ice_servers_shim(val);
        self
    }
    #[cfg(feature = "RtcIceTransportPolicy")]
    #[doc = "Change the `iceTransportPolicy` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcConfiguration`, `RtcIceTransportPolicy`*"]
    pub fn ice_transport_policy(&mut self, val: RtcIceTransportPolicy) -> &mut Self {
        self.ice_transport_policy_shim(val);
        self
    }
    #[doc = "Change the `peerIdentity` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcConfiguration`*"]
    pub fn peer_identity(&mut self, val: Option<&str>) -> &mut Self {
        self.peer_identity_shim(val);
        self
    }
}
impl Default for RtcConfiguration {
    fn default() -> Self {
        Self::new()
    }
}
