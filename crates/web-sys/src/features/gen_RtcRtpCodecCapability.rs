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
    #[wasm_bindgen(method, getter = "channels")]
    fn channels_shim(this: &RtcRtpCodecCapability) -> u16;
    #[wasm_bindgen(method, setter = "channels")]
    fn set_channels_shim(this: &RtcRtpCodecCapability, val: u16);
    #[wasm_bindgen(method, getter = "clockRate")]
    fn clock_rate_shim(this: &RtcRtpCodecCapability) -> u32;
    #[wasm_bindgen(method, setter = "clockRate")]
    fn set_clock_rate_shim(this: &RtcRtpCodecCapability, val: u32);
    #[wasm_bindgen(method, getter = "mimeType")]
    fn mime_type_shim(this: &RtcRtpCodecCapability) -> String;
    #[wasm_bindgen(method, setter = "mimeType")]
    fn set_mime_type_shim(this: &RtcRtpCodecCapability, val: &str);
    #[wasm_bindgen(method, getter = "sdpFmtpLine")]
    fn sdp_fmtp_line_shim(this: &RtcRtpCodecCapability) -> String;
    #[wasm_bindgen(method, setter = "sdpFmtpLine")]
    fn set_sdp_fmtp_line_shim(this: &RtcRtpCodecCapability, val: &str);
}
#[doc = "The trait to access properties on the `RtcRtpCodecCapability` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecCapability`*"]
pub trait RtcRtpCodecCapabilityGetters {
    #[doc = "Get the `channels` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecCapability`*"]
    fn channels(&self) -> u16;
    #[doc = "Get the `clockRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecCapability`*"]
    fn clock_rate(&self) -> u32;
    #[doc = "Get the `mimeType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecCapability`*"]
    fn mime_type(&self) -> String;
    #[doc = "Get the `sdpFmtpLine` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecCapability`*"]
    fn sdp_fmtp_line(&self) -> String;
}
impl RtcRtpCodecCapabilityGetters for RtcRtpCodecCapability {
    fn channels(&self) -> u16 {
        self.channels_shim()
    }
    fn clock_rate(&self) -> u32 {
        self.clock_rate_shim()
    }
    fn mime_type(&self) -> String {
        self.mime_type_shim()
    }
    fn sdp_fmtp_line(&self) -> String {
        self.sdp_fmtp_line_shim()
    }
}
impl RtcRtpCodecCapability {
    #[doc = "Construct a new `RtcRtpCodecCapability`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecCapability`*"]
    pub fn new(clock_rate: u32, mime_type: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        Self::clock_rate(&mut ret, clock_rate);
        Self::mime_type(&mut ret, mime_type);
        ret
    }
    #[doc = "Change the `channels` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecCapability`*"]
    pub fn channels(&mut self, val: u16) -> &mut Self {
        self.set_channels_shim(val);
        self
    }
    #[doc = "Change the `clockRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecCapability`*"]
    pub fn clock_rate(&mut self, val: u32) -> &mut Self {
        self.set_clock_rate_shim(val);
        self
    }
    #[doc = "Change the `mimeType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecCapability`*"]
    pub fn mime_type(&mut self, val: &str) -> &mut Self {
        self.set_mime_type_shim(val);
        self
    }
    #[doc = "Change the `sdpFmtpLine` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecCapability`*"]
    pub fn sdp_fmtp_line(&mut self, val: &str) -> &mut Self {
        self.set_sdp_fmtp_line_shim(val);
        self
    }
}
