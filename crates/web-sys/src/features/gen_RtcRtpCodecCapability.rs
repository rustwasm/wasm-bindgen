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
    #[doc = "Get the `channels` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecCapability`*"]
    #[wasm_bindgen(method, getter = "channels")]
    pub fn get_channels(this: &RtcRtpCodecCapability) -> Option<u16>;
    #[wasm_bindgen(method, setter = "channels")]
    fn set_channels(this: &RtcRtpCodecCapability, val: u16);
    #[doc = "Get the `clockRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecCapability`*"]
    #[wasm_bindgen(method, getter = "clockRate")]
    pub fn get_clock_rate(this: &RtcRtpCodecCapability) -> u32;
    #[wasm_bindgen(method, setter = "clockRate")]
    fn set_clock_rate(this: &RtcRtpCodecCapability, val: u32);
    #[doc = "Get the `mimeType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecCapability`*"]
    #[wasm_bindgen(method, getter = "mimeType")]
    pub fn get_mime_type(this: &RtcRtpCodecCapability) -> String;
    #[wasm_bindgen(method, setter = "mimeType")]
    fn set_mime_type(this: &RtcRtpCodecCapability, val: &str);
    #[doc = "Get the `sdpFmtpLine` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecCapability`*"]
    #[wasm_bindgen(method, getter = "sdpFmtpLine")]
    pub fn get_sdp_fmtp_line(this: &RtcRtpCodecCapability) -> Option<String>;
    #[wasm_bindgen(method, setter = "sdpFmtpLine")]
    fn set_sdp_fmtp_line(this: &RtcRtpCodecCapability, val: &str);
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
        self.set_channels(val);
        self
    }
    #[doc = "Change the `clockRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecCapability`*"]
    pub fn clock_rate(&mut self, val: u32) -> &mut Self {
        self.set_clock_rate(val);
        self
    }
    #[doc = "Change the `mimeType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecCapability`*"]
    pub fn mime_type(&mut self, val: &str) -> &mut Self {
        self.set_mime_type(val);
        self
    }
    #[doc = "Change the `sdpFmtpLine` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecCapability`*"]
    pub fn sdp_fmtp_line(&mut self, val: &str) -> &mut Self {
        self.set_sdp_fmtp_line(val);
        self
    }
}
