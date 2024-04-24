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
    #[wasm_bindgen(method, getter = "bundlePolicy")]
    fn bundle_policy_shim(this: &RtcConfiguration) -> RtcBundlePolicy;
    #[cfg(feature = "RtcBundlePolicy")]
    #[wasm_bindgen(method, setter = "bundlePolicy")]
    fn set_bundle_policy_shim(this: &RtcConfiguration, val: RtcBundlePolicy);
    #[wasm_bindgen(method, getter = "certificates")]
    fn certificates_shim(this: &RtcConfiguration) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "certificates")]
    fn set_certificates_shim(this: &RtcConfiguration, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "iceServers")]
    fn ice_servers_shim(this: &RtcConfiguration) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "iceServers")]
    fn set_ice_servers_shim(this: &RtcConfiguration, val: &::wasm_bindgen::JsValue);
    #[cfg(feature = "RtcIceTransportPolicy")]
    #[wasm_bindgen(method, getter = "iceTransportPolicy")]
    fn ice_transport_policy_shim(this: &RtcConfiguration) -> RtcIceTransportPolicy;
    #[cfg(feature = "RtcIceTransportPolicy")]
    #[wasm_bindgen(method, setter = "iceTransportPolicy")]
    fn set_ice_transport_policy_shim(this: &RtcConfiguration, val: RtcIceTransportPolicy);
    #[wasm_bindgen(method, getter = "peerIdentity")]
    fn peer_identity_shim(this: &RtcConfiguration) -> Option<String>;
    #[wasm_bindgen(method, setter = "peerIdentity")]
    fn set_peer_identity_shim(this: &RtcConfiguration, val: Option<&str>);
}
#[doc = "The trait to access properties on the `RtcConfiguration` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcConfiguration`*"]
pub trait RtcConfigurationGetters {
    #[cfg(feature = "RtcBundlePolicy")]
    #[doc = "Get the `bundlePolicy` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcBundlePolicy`, `RtcConfiguration`*"]
    fn bundle_policy(&self) -> RtcBundlePolicy;
    #[doc = "Get the `certificates` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcConfiguration`*"]
    fn certificates(&self) -> ::js_sys::Array;
    #[doc = "Get the `iceServers` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcConfiguration`*"]
    fn ice_servers(&self) -> ::js_sys::Array;
    #[cfg(feature = "RtcIceTransportPolicy")]
    #[doc = "Get the `iceTransportPolicy` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcConfiguration`, `RtcIceTransportPolicy`*"]
    fn ice_transport_policy(&self) -> RtcIceTransportPolicy;
    #[doc = "Get the `peerIdentity` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcConfiguration`*"]
    fn peer_identity(&self) -> Option<String>;
}
impl RtcConfigurationGetters for RtcConfiguration {
    #[cfg(feature = "RtcBundlePolicy")]
    fn bundle_policy(&self) -> RtcBundlePolicy {
        self.bundle_policy_shim()
    }
    fn certificates(&self) -> ::js_sys::Array {
        self.certificates_shim()
    }
    fn ice_servers(&self) -> ::js_sys::Array {
        self.ice_servers_shim()
    }
    #[cfg(feature = "RtcIceTransportPolicy")]
    fn ice_transport_policy(&self) -> RtcIceTransportPolicy {
        self.ice_transport_policy_shim()
    }
    fn peer_identity(&self) -> Option<String> {
        self.peer_identity_shim()
    }
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
        self.set_bundle_policy_shim(val);
        self
    }
    #[doc = "Change the `certificates` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcConfiguration`*"]
    pub fn certificates(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_certificates_shim(val);
        self
    }
    #[doc = "Change the `iceServers` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcConfiguration`*"]
    pub fn ice_servers(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_ice_servers_shim(val);
        self
    }
    #[cfg(feature = "RtcIceTransportPolicy")]
    #[doc = "Change the `iceTransportPolicy` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcConfiguration`, `RtcIceTransportPolicy`*"]
    pub fn ice_transport_policy(&mut self, val: RtcIceTransportPolicy) -> &mut Self {
        self.set_ice_transport_policy_shim(val);
        self
    }
    #[doc = "Change the `peerIdentity` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcConfiguration`*"]
    pub fn peer_identity(&mut self, val: Option<&str>) -> &mut Self {
        self.set_peer_identity_shim(val);
        self
    }
}
impl Default for RtcConfiguration {
    fn default() -> Self {
        Self::new()
    }
}
