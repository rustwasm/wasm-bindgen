#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCSessionDescriptionInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcSessionDescriptionInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcSessionDescriptionInit`*"]
    pub type RtcSessionDescriptionInit;
    #[wasm_bindgen(method, getter = "sdp")]
    fn sdp_shim(this: &RtcSessionDescriptionInit) -> &str;
    #[wasm_bindgen(method, setter = "sdp")]
    fn set_sdp_shim(this: &RtcSessionDescriptionInit, val: &str);
    #[cfg(feature = "RtcSdpType")]
    #[wasm_bindgen(method, getter = "type")]
    fn type__shim(this: &RtcSessionDescriptionInit) -> RtcSdpType;
    #[cfg(feature = "RtcSdpType")]
    #[wasm_bindgen(method, setter = "type")]
    fn set_type__shim(this: &RtcSessionDescriptionInit, val: RtcSdpType);
}
#[doc = "The trait to access properties on the `RtcSessionDescriptionInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcSessionDescriptionInit`*"]
pub trait RtcSessionDescriptionInitGetters {
    #[doc = "Get the `sdp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcSessionDescriptionInit`*"]
    fn sdp(&self) -> &str;
    #[cfg(feature = "RtcSdpType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcSdpType`, `RtcSessionDescriptionInit`*"]
    fn type_(&self) -> RtcSdpType;
}
impl RtcSessionDescriptionInitGetters for RtcSessionDescriptionInit {
    fn sdp(&self) -> &str {
        self.sdp_shim()
    }
    #[cfg(feature = "RtcSdpType")]
    fn type_(&self) -> RtcSdpType {
        self.type__shim()
    }
}
impl RtcSessionDescriptionInit {
    #[cfg(feature = "RtcSdpType")]
    #[doc = "Construct a new `RtcSessionDescriptionInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcSdpType`, `RtcSessionDescriptionInit`*"]
    pub fn new(type_: RtcSdpType) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.type_(type_);
        ret
    }
    #[doc = "Change the `sdp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcSessionDescriptionInit`*"]
    pub fn sdp(&mut self, val: &str) -> &mut Self {
        self.set_sdp_shim(val);
        self
    }
    #[cfg(feature = "RtcSdpType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcSdpType`, `RtcSessionDescriptionInit`*"]
    pub fn type_(&mut self, val: RtcSdpType) -> &mut Self {
        self.set_type__shim(val);
        self
    }
}
