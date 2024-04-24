#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCIdentityProviderDetails)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcIdentityProviderDetails` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProviderDetails`*"]
    pub type RtcIdentityProviderDetails;
    #[wasm_bindgen(method, getter = "domain")]
    fn domain_shim(this: &RtcIdentityProviderDetails) -> String;
    #[wasm_bindgen(method, setter = "domain")]
    fn set_domain_shim(this: &RtcIdentityProviderDetails, val: &str);
    #[wasm_bindgen(method, getter = "protocol")]
    fn protocol_shim(this: &RtcIdentityProviderDetails) -> String;
    #[wasm_bindgen(method, setter = "protocol")]
    fn set_protocol_shim(this: &RtcIdentityProviderDetails, val: &str);
}
#[doc = "The trait to access properties on the `RtcIdentityProviderDetails` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcIdentityProviderDetails`*"]
pub trait RtcIdentityProviderDetailsGetters {
    #[doc = "Get the `domain` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProviderDetails`*"]
    fn domain(&self) -> String;
    #[doc = "Get the `protocol` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProviderDetails`*"]
    fn protocol(&self) -> String;
}
impl RtcIdentityProviderDetailsGetters for RtcIdentityProviderDetails {
    fn domain(&self) -> String {
        self.domain_shim()
    }
    fn protocol(&self) -> String {
        self.protocol_shim()
    }
}
impl RtcIdentityProviderDetails {
    #[doc = "Construct a new `RtcIdentityProviderDetails`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProviderDetails`*"]
    pub fn new(domain: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        Self::domain(&mut ret, domain);
        ret
    }
    #[doc = "Change the `domain` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProviderDetails`*"]
    pub fn domain(&mut self, val: &str) -> &mut Self {
        self.set_domain_shim(val);
        self
    }
    #[doc = "Change the `protocol` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProviderDetails`*"]
    pub fn protocol(&mut self, val: &str) -> &mut Self {
        self.set_protocol_shim(val);
        self
    }
}
