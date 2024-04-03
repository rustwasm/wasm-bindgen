#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCRtpCodecCapability)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcRtpCodecCapability` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecCapability`*"]
    pub type RtcRtpCodecCapability;
    #[wasm_bindgen(method, setter = "channels")]
    fn channels_shim(this: &RtcRtpCodecCapability, val: u16);
    #[wasm_bindgen(method, setter = "clockRate")]
    fn clock_rate_shim(this: &RtcRtpCodecCapability, val: u32);
    #[wasm_bindgen(method, setter = "mimeType")]
    fn mime_type_shim(this: &RtcRtpCodecCapability, val: &str);
    #[wasm_bindgen(method, setter = "sdpFmtpLine")]
    fn sdp_fmtp_line_shim(this: &RtcRtpCodecCapability, val: &str);
}
impl RtcRtpCodecCapability {
    #[doc = "Construct a new `RtcRtpCodecCapability`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecCapability`*"]
    pub fn new(clock_rate: u32, mime_type: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.clock_rate(clock_rate);
        ret.mime_type(mime_type);
        ret
    }
    #[doc = "Change the `channels` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecCapability`*"]
    pub fn channels(&mut self, val: u16) -> &mut Self {
        self.channels_shim(val);
        self
    }
    #[doc = "Change the `clockRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecCapability`*"]
    pub fn clock_rate(&mut self, val: u32) -> &mut Self {
        self.clock_rate_shim(val);
        self
    }
    #[doc = "Change the `mimeType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecCapability`*"]
    pub fn mime_type(&mut self, val: &str) -> &mut Self {
        self.mime_type_shim(val);
        self
    }
    #[doc = "Change the `sdpFmtpLine` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecCapability`*"]
    pub fn sdp_fmtp_line(&mut self, val: &str) -> &mut Self {
        self.sdp_fmtp_line_shim(val);
        self
    }
}
