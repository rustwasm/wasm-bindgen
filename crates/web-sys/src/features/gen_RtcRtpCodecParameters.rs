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
    #[wasm_bindgen(method, getter = "channels")]
    fn channels_shim(this: &RtcRtpCodecParameters) -> u16;
    #[wasm_bindgen(method, setter = "channels")]
    fn set_channels_shim(this: &RtcRtpCodecParameters, val: u16);
    #[wasm_bindgen(method, getter = "clockRate")]
    fn clock_rate_shim(this: &RtcRtpCodecParameters) -> u32;
    #[wasm_bindgen(method, setter = "clockRate")]
    fn set_clock_rate_shim(this: &RtcRtpCodecParameters, val: u32);
    #[wasm_bindgen(method, getter = "mimeType")]
    fn mime_type_shim(this: &RtcRtpCodecParameters) -> String;
    #[wasm_bindgen(method, setter = "mimeType")]
    fn set_mime_type_shim(this: &RtcRtpCodecParameters, val: &str);
    #[wasm_bindgen(method, getter = "payloadType")]
    fn payload_type_shim(this: &RtcRtpCodecParameters) -> u16;
    #[wasm_bindgen(method, setter = "payloadType")]
    fn set_payload_type_shim(this: &RtcRtpCodecParameters, val: u16);
    #[wasm_bindgen(method, getter = "sdpFmtpLine")]
    fn sdp_fmtp_line_shim(this: &RtcRtpCodecParameters) -> String;
    #[wasm_bindgen(method, setter = "sdpFmtpLine")]
    fn set_sdp_fmtp_line_shim(this: &RtcRtpCodecParameters, val: &str);
}
#[doc = "The trait to access properties on the `RtcRtpCodecParameters` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecParameters`*"]
pub trait RtcRtpCodecParametersGetters {
    #[doc = "Get the `channels` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecParameters`*"]
    fn channels(&self) -> u16;
    #[doc = "Get the `clockRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecParameters`*"]
    fn clock_rate(&self) -> u32;
    #[doc = "Get the `mimeType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecParameters`*"]
    fn mime_type(&self) -> String;
    #[doc = "Get the `payloadType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecParameters`*"]
    fn payload_type(&self) -> u16;
    #[doc = "Get the `sdpFmtpLine` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecParameters`*"]
    fn sdp_fmtp_line(&self) -> String;
}
impl RtcRtpCodecParametersGetters for RtcRtpCodecParameters {
    fn channels(&self) -> u16 {
        self.channels_shim()
    }
    fn clock_rate(&self) -> u32 {
        self.clock_rate_shim()
    }
    fn mime_type(&self) -> String {
        self.mime_type_shim()
    }
    fn payload_type(&self) -> u16 {
        self.payload_type_shim()
    }
    fn sdp_fmtp_line(&self) -> String {
        self.sdp_fmtp_line_shim()
    }
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
        self.set_channels_shim(val);
        self
    }
    #[doc = "Change the `clockRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecParameters`*"]
    pub fn clock_rate(&mut self, val: u32) -> &mut Self {
        self.set_clock_rate_shim(val);
        self
    }
    #[doc = "Change the `mimeType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecParameters`*"]
    pub fn mime_type(&mut self, val: &str) -> &mut Self {
        self.set_mime_type_shim(val);
        self
    }
    #[doc = "Change the `payloadType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecParameters`*"]
    pub fn payload_type(&mut self, val: u16) -> &mut Self {
        self.set_payload_type_shim(val);
        self
    }
    #[doc = "Change the `sdpFmtpLine` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecParameters`*"]
    pub fn sdp_fmtp_line(&mut self, val: &str) -> &mut Self {
        self.set_sdp_fmtp_line_shim(val);
        self
    }
}
impl Default for RtcRtpCodecParameters {
    fn default() -> Self {
        Self::new()
    }
}
