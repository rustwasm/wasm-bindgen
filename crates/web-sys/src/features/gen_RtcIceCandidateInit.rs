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
    #[doc = "Get the `candidate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateInit`*"]
    #[wasm_bindgen(method, getter = "candidate")]
    pub fn get_candidate(this: &RtcIceCandidateInit) -> String;
    #[wasm_bindgen(method, setter = "candidate")]
    fn set_candidate(this: &RtcIceCandidateInit, val: &str);
    #[doc = "Get the `sdpMLineIndex` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateInit`*"]
    #[wasm_bindgen(method, getter = "sdpMLineIndex")]
    pub fn get_sdp_m_line_index(this: &RtcIceCandidateInit) -> Option<u16>;
    #[wasm_bindgen(method, setter = "sdpMLineIndex")]
    fn set_sdp_m_line_index(this: &RtcIceCandidateInit, val: Option<u16>);
    #[doc = "Get the `sdpMid` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateInit`*"]
    #[wasm_bindgen(method, getter = "sdpMid")]
    pub fn get_sdp_mid(this: &RtcIceCandidateInit) -> Option<String>;
    #[wasm_bindgen(method, setter = "sdpMid")]
    fn set_sdp_mid(this: &RtcIceCandidateInit, val: Option<&str>);
}
impl RtcIceCandidateInit {
    #[doc = "Construct a new `RtcIceCandidateInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateInit`*"]
    pub fn new(candidate: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.candidate(candidate);
        ret
    }
    #[doc = "Change the `candidate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateInit`*"]
    pub fn candidate(&mut self, val: &str) -> &mut Self {
        self.set_candidate(val);
        self
    }
    #[doc = "Change the `sdpMLineIndex` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateInit`*"]
    pub fn sdp_m_line_index(&mut self, val: Option<u16>) -> &mut Self {
        self.set_sdp_m_line_index(val);
        self
    }
    #[doc = "Change the `sdpMid` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateInit`*"]
    pub fn sdp_mid(&mut self, val: Option<&str>) -> &mut Self {
        self.set_sdp_mid(val);
        self
    }
}
