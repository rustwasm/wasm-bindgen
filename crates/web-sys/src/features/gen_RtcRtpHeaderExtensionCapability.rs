#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCRtpHeaderExtensionCapability)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcRtpHeaderExtensionCapability` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpHeaderExtensionCapability`*"]
    pub type RtcRtpHeaderExtensionCapability;
    #[wasm_bindgen(method, getter = "uri")]
    fn uri_shim(this: &RtcRtpHeaderExtensionCapability) -> &str;
    #[wasm_bindgen(method, setter = "uri")]
    fn set_uri_shim(this: &RtcRtpHeaderExtensionCapability, val: &str);
}
#[doc = "The trait to access properties on the `RtcRtpHeaderExtensionCapability` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcRtpHeaderExtensionCapability`*"]
pub trait RtcRtpHeaderExtensionCapabilityGetters {
    #[doc = "Get the `uri` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpHeaderExtensionCapability`*"]
    fn uri(&self) -> &str;
}
impl RtcRtpHeaderExtensionCapabilityGetters for RtcRtpHeaderExtensionCapability {
    fn uri(&self) -> &str {
        self.uri_shim()
    }
}
impl RtcRtpHeaderExtensionCapability {
    #[doc = "Construct a new `RtcRtpHeaderExtensionCapability`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpHeaderExtensionCapability`*"]
    pub fn new(uri: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.uri(uri);
        ret
    }
    #[doc = "Change the `uri` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpHeaderExtensionCapability`*"]
    pub fn uri(&mut self, val: &str) -> &mut Self {
        self.set_uri_shim(val);
        self
    }
}
