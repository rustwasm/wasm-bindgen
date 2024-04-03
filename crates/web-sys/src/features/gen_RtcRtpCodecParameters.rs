#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCRtpCodecParameters)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcRtpCodecParameters` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecParameters`*"]
    pub type RtcRtpCodecParameters;
    #[wasm_bindgen(method, setter = "channels")]
    fn channels_shim(this: &RtcRtpCodecParameters, val: u16);
    #[wasm_bindgen(method, setter = "clockRate")]
    fn clock_rate_shim(this: &RtcRtpCodecParameters, val: u32);
    #[wasm_bindgen(method, setter = "mimeType")]
    fn mime_type_shim(this: &RtcRtpCodecParameters, val: &str);
    #[wasm_bindgen(method, setter = "payloadType")]
    fn payload_type_shim(this: &RtcRtpCodecParameters, val: u16);
    #[wasm_bindgen(method, setter = "sdpFmtpLine")]
    fn sdp_fmtp_line_shim(this: &RtcRtpCodecParameters, val: &str);
}
impl RtcRtpCodecParameters {
    #[doc = "Construct a new `RtcRtpCodecParameters`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecParameters`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `channels` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecParameters`*"]
    pub fn channels(&mut self, val: u16) -> &mut Self {
        self.channels_shim(val);
        self
    }
    #[doc = "Change the `clockRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecParameters`*"]
    pub fn clock_rate(&mut self, val: u32) -> &mut Self {
        self.clock_rate_shim(val);
        self
    }
    #[doc = "Change the `mimeType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecParameters`*"]
    pub fn mime_type(&mut self, val: &str) -> &mut Self {
        self.mime_type_shim(val);
        self
    }
    #[doc = "Change the `payloadType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecParameters`*"]
    pub fn payload_type(&mut self, val: u16) -> &mut Self {
        self.payload_type_shim(val);
        self
    }
    #[doc = "Change the `sdpFmtpLine` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecParameters`*"]
    pub fn sdp_fmtp_line(&mut self, val: &str) -> &mut Self {
        self.sdp_fmtp_line_shim(val);
        self
    }
}
impl Default for RtcRtpCodecParameters {
    fn default() -> Self {
        Self::new()
    }
}
