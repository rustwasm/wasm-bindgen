#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCRtpEncodingParameters)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcRtpEncodingParameters` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`*"]
    pub type RtcRtpEncodingParameters;
    #[wasm_bindgen(method, setter = "active")]
    fn active_shim(this: &RtcRtpEncodingParameters, val: bool);
    #[cfg(feature = "RtcDegradationPreference")]
    #[wasm_bindgen(method, setter = "degradationPreference")]
    fn degradation_preference_shim(this: &RtcRtpEncodingParameters, val: RtcDegradationPreference);
    #[cfg(feature = "RtcFecParameters")]
    #[wasm_bindgen(method, setter = "fec")]
    fn fec_shim(this: &RtcRtpEncodingParameters, val: &RtcFecParameters);
    #[wasm_bindgen(method, setter = "maxBitrate")]
    fn max_bitrate_shim(this: &RtcRtpEncodingParameters, val: u32);
    #[cfg(feature = "RtcPriorityType")]
    #[wasm_bindgen(method, setter = "priority")]
    fn priority_shim(this: &RtcRtpEncodingParameters, val: RtcPriorityType);
    #[wasm_bindgen(method, setter = "rid")]
    fn rid_shim(this: &RtcRtpEncodingParameters, val: &str);
    #[cfg(feature = "RtcRtxParameters")]
    #[wasm_bindgen(method, setter = "rtx")]
    fn rtx_shim(this: &RtcRtpEncodingParameters, val: &RtcRtxParameters);
    #[wasm_bindgen(method, setter = "scalabilityMode")]
    fn scalability_mode_shim(this: &RtcRtpEncodingParameters, val: &str);
    #[wasm_bindgen(method, setter = "scaleResolutionDownBy")]
    fn scale_resolution_down_by_shim(this: &RtcRtpEncodingParameters, val: f32);
    #[wasm_bindgen(method, setter = "ssrc")]
    fn ssrc_shim(this: &RtcRtpEncodingParameters, val: u32);
}
impl RtcRtpEncodingParameters {
    #[doc = "Construct a new `RtcRtpEncodingParameters`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `active` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`*"]
    pub fn active(&mut self, val: bool) -> &mut Self {
        self.active_shim(val);
        self
    }
    #[cfg(feature = "RtcDegradationPreference")]
    #[doc = "Change the `degradationPreference` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDegradationPreference`, `RtcRtpEncodingParameters`*"]
    pub fn degradation_preference(&mut self, val: RtcDegradationPreference) -> &mut Self {
        self.degradation_preference_shim(val);
        self
    }
    #[cfg(feature = "RtcFecParameters")]
    #[doc = "Change the `fec` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcFecParameters`, `RtcRtpEncodingParameters`*"]
    pub fn fec(&mut self, val: &RtcFecParameters) -> &mut Self {
        self.fec_shim(val);
        self
    }
    #[doc = "Change the `maxBitrate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`*"]
    pub fn max_bitrate(&mut self, val: u32) -> &mut Self {
        self.max_bitrate_shim(val);
        self
    }
    #[cfg(feature = "RtcPriorityType")]
    #[doc = "Change the `priority` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPriorityType`, `RtcRtpEncodingParameters`*"]
    pub fn priority(&mut self, val: RtcPriorityType) -> &mut Self {
        self.priority_shim(val);
        self
    }
    #[doc = "Change the `rid` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`*"]
    pub fn rid(&mut self, val: &str) -> &mut Self {
        self.rid_shim(val);
        self
    }
    #[cfg(feature = "RtcRtxParameters")]
    #[doc = "Change the `rtx` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`, `RtcRtxParameters`*"]
    pub fn rtx(&mut self, val: &RtcRtxParameters) -> &mut Self {
        self.rtx_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `scalabilityMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn scalability_mode(&mut self, val: &str) -> &mut Self {
        self.scalability_mode_shim(val);
        self
    }
    #[doc = "Change the `scaleResolutionDownBy` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`*"]
    pub fn scale_resolution_down_by(&mut self, val: f32) -> &mut Self {
        self.scale_resolution_down_by_shim(val);
        self
    }
    #[doc = "Change the `ssrc` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`*"]
    pub fn ssrc(&mut self, val: u32) -> &mut Self {
        self.ssrc_shim(val);
        self
    }
}
impl Default for RtcRtpEncodingParameters {
    fn default() -> Self {
        Self::new()
    }
}
