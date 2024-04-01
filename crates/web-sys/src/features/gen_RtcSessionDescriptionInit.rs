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
    #[wasm_bindgen(method, setter = "sdp")]
    fn sdp_shim(this: &RtcSessionDescriptionInit, val: &str);
    #[cfg(feature = "RtcSdpType")]
    #[wasm_bindgen(method, setter = "type")]
    fn type__shim(this: &RtcSessionDescriptionInit, val: RtcSdpType);
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
        self.sdp_shim(val);
        self
    }
    #[cfg(feature = "RtcSdpType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcSdpType`, `RtcSessionDescriptionInit`*"]
    pub fn type_(&mut self, val: RtcSdpType) -> &mut Self {
        self.type__shim(val);
        self
    }
}
