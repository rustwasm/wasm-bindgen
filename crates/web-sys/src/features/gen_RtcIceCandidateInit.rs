#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCIceCandidateInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcIceCandidateInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateInit`*"]
    pub type RtcIceCandidateInit;
    #[wasm_bindgen(method, getter = "candidate")]
    fn candidate_shim(this: &RtcIceCandidateInit) -> String;
    #[wasm_bindgen(method, setter = "candidate")]
    fn set_candidate_shim(this: &RtcIceCandidateInit, val: &str);
    #[wasm_bindgen(method, getter = "sdpMLineIndex")]
    fn sdp_m_line_index_shim(this: &RtcIceCandidateInit) -> Option<u16>;
    #[wasm_bindgen(method, setter = "sdpMLineIndex")]
    fn set_sdp_m_line_index_shim(this: &RtcIceCandidateInit, val: Option<u16>);
    #[wasm_bindgen(method, getter = "sdpMid")]
    fn sdp_mid_shim(this: &RtcIceCandidateInit) -> Option<String>;
    #[wasm_bindgen(method, setter = "sdpMid")]
    fn set_sdp_mid_shim(this: &RtcIceCandidateInit, val: Option<&str>);
}
#[doc = "The trait to access properties on the `RtcIceCandidateInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateInit`*"]
pub trait RtcIceCandidateInitGetters {
    #[doc = "Get the `candidate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateInit`*"]
    fn candidate(&self) -> String;
    #[doc = "Get the `sdpMLineIndex` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateInit`*"]
    fn sdp_m_line_index(&self) -> Option<u16>;
    #[doc = "Get the `sdpMid` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateInit`*"]
    fn sdp_mid(&self) -> Option<String>;
}
impl RtcIceCandidateInitGetters for RtcIceCandidateInit {
    fn candidate(&self) -> String {
        self.candidate_shim()
    }
    fn sdp_m_line_index(&self) -> Option<u16> {
        self.sdp_m_line_index_shim()
    }
    fn sdp_mid(&self) -> Option<String> {
        self.sdp_mid_shim()
    }
}
impl RtcIceCandidateInit {
    #[doc = "Construct a new `RtcIceCandidateInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateInit`*"]
    pub fn new(candidate: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        Self::candidate(&mut ret, candidate);
        ret
    }
    #[doc = "Change the `candidate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateInit`*"]
    pub fn candidate(&mut self, val: &str) -> &mut Self {
        self.set_candidate_shim(val);
        self
    }
    #[doc = "Change the `sdpMLineIndex` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateInit`*"]
    pub fn sdp_m_line_index(&mut self, val: Option<u16>) -> &mut Self {
        self.set_sdp_m_line_index_shim(val);
        self
    }
    #[doc = "Change the `sdpMid` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateInit`*"]
    pub fn sdp_mid(&mut self, val: Option<&str>) -> &mut Self {
        self.set_sdp_mid_shim(val);
        self
    }
}
