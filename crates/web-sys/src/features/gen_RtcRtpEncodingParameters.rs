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
    #[wasm_bindgen(method, getter = "active")]
    fn active_shim(this: &RtcRtpEncodingParameters) -> bool;
    #[wasm_bindgen(method, setter = "active")]
    fn set_active_shim(this: &RtcRtpEncodingParameters, val: bool);
    #[cfg(feature = "RtcDegradationPreference")]
    #[wasm_bindgen(method, getter = "degradationPreference")]
    fn degradation_preference_shim(this: &RtcRtpEncodingParameters) -> RtcDegradationPreference;
    #[cfg(feature = "RtcDegradationPreference")]
    #[wasm_bindgen(method, setter = "degradationPreference")]
    fn set_degradation_preference_shim(
        this: &RtcRtpEncodingParameters,
        val: RtcDegradationPreference,
    );
    #[cfg(feature = "RtcFecParameters")]
    #[wasm_bindgen(method, getter = "fec")]
    fn fec_shim(this: &RtcRtpEncodingParameters) -> RtcFecParameters;
    #[cfg(feature = "RtcFecParameters")]
    #[wasm_bindgen(method, setter = "fec")]
    fn set_fec_shim(this: &RtcRtpEncodingParameters, val: &RtcFecParameters);
    #[wasm_bindgen(method, getter = "maxBitrate")]
    fn max_bitrate_shim(this: &RtcRtpEncodingParameters) -> u32;
    #[wasm_bindgen(method, setter = "maxBitrate")]
    fn set_max_bitrate_shim(this: &RtcRtpEncodingParameters, val: u32);
    #[cfg(feature = "RtcPriorityType")]
    #[wasm_bindgen(method, getter = "priority")]
    fn priority_shim(this: &RtcRtpEncodingParameters) -> RtcPriorityType;
    #[cfg(feature = "RtcPriorityType")]
    #[wasm_bindgen(method, setter = "priority")]
    fn set_priority_shim(this: &RtcRtpEncodingParameters, val: RtcPriorityType);
    #[wasm_bindgen(method, getter = "rid")]
    fn rid_shim(this: &RtcRtpEncodingParameters) -> String;
    #[wasm_bindgen(method, setter = "rid")]
    fn set_rid_shim(this: &RtcRtpEncodingParameters, val: &str);
    #[cfg(feature = "RtcRtxParameters")]
    #[wasm_bindgen(method, getter = "rtx")]
    fn rtx_shim(this: &RtcRtpEncodingParameters) -> RtcRtxParameters;
    #[cfg(feature = "RtcRtxParameters")]
    #[wasm_bindgen(method, setter = "rtx")]
    fn set_rtx_shim(this: &RtcRtpEncodingParameters, val: &RtcRtxParameters);
    #[wasm_bindgen(method, getter = "scalabilityMode")]
    fn scalability_mode_shim(this: &RtcRtpEncodingParameters) -> String;
    #[wasm_bindgen(method, setter = "scalabilityMode")]
    fn set_scalability_mode_shim(this: &RtcRtpEncodingParameters, val: &str);
    #[wasm_bindgen(method, getter = "scaleResolutionDownBy")]
    fn scale_resolution_down_by_shim(this: &RtcRtpEncodingParameters) -> f32;
    #[wasm_bindgen(method, setter = "scaleResolutionDownBy")]
    fn set_scale_resolution_down_by_shim(this: &RtcRtpEncodingParameters, val: f32);
    #[wasm_bindgen(method, getter = "ssrc")]
    fn ssrc_shim(this: &RtcRtpEncodingParameters) -> u32;
    #[wasm_bindgen(method, setter = "ssrc")]
    fn set_ssrc_shim(this: &RtcRtpEncodingParameters, val: u32);
}
#[doc = "The trait to access properties on the `RtcRtpEncodingParameters` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`*"]
pub trait RtcRtpEncodingParametersGetters {
    #[doc = "Get the `active` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`*"]
    fn active(&self) -> bool;
    #[cfg(feature = "RtcDegradationPreference")]
    #[doc = "Get the `degradationPreference` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDegradationPreference`, `RtcRtpEncodingParameters`*"]
    fn degradation_preference(&self) -> RtcDegradationPreference;
    #[cfg(feature = "RtcFecParameters")]
    #[doc = "Get the `fec` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcFecParameters`, `RtcRtpEncodingParameters`*"]
    fn fec(&self) -> RtcFecParameters;
    #[doc = "Get the `maxBitrate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`*"]
    fn max_bitrate(&self) -> u32;
    #[cfg(feature = "RtcPriorityType")]
    #[doc = "Get the `priority` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPriorityType`, `RtcRtpEncodingParameters`*"]
    fn priority(&self) -> RtcPriorityType;
    #[doc = "Get the `rid` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`*"]
    fn rid(&self) -> String;
    #[cfg(feature = "RtcRtxParameters")]
    #[doc = "Get the `rtx` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`, `RtcRtxParameters`*"]
    fn rtx(&self) -> RtcRtxParameters;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `scalabilityMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn scalability_mode(&self) -> String;
    #[doc = "Get the `scaleResolutionDownBy` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`*"]
    fn scale_resolution_down_by(&self) -> f32;
    #[doc = "Get the `ssrc` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`*"]
    fn ssrc(&self) -> u32;
}
impl RtcRtpEncodingParametersGetters for RtcRtpEncodingParameters {
    fn active(&self) -> bool {
        self.active_shim()
    }
    #[cfg(feature = "RtcDegradationPreference")]
    fn degradation_preference(&self) -> RtcDegradationPreference {
        self.degradation_preference_shim()
    }
    #[cfg(feature = "RtcFecParameters")]
    fn fec(&self) -> RtcFecParameters {
        self.fec_shim()
    }
    fn max_bitrate(&self) -> u32 {
        self.max_bitrate_shim()
    }
    #[cfg(feature = "RtcPriorityType")]
    fn priority(&self) -> RtcPriorityType {
        self.priority_shim()
    }
    fn rid(&self) -> String {
        self.rid_shim()
    }
    #[cfg(feature = "RtcRtxParameters")]
    fn rtx(&self) -> RtcRtxParameters {
        self.rtx_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn scalability_mode(&self) -> String {
        self.scalability_mode_shim()
    }
    fn scale_resolution_down_by(&self) -> f32 {
        self.scale_resolution_down_by_shim()
    }
    fn ssrc(&self) -> u32 {
        self.ssrc_shim()
    }
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
        self.set_active_shim(val);
        self
    }
    #[cfg(feature = "RtcDegradationPreference")]
    #[doc = "Change the `degradationPreference` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDegradationPreference`, `RtcRtpEncodingParameters`*"]
    pub fn degradation_preference(&mut self, val: RtcDegradationPreference) -> &mut Self {
        self.set_degradation_preference_shim(val);
        self
    }
    #[cfg(feature = "RtcFecParameters")]
    #[doc = "Change the `fec` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcFecParameters`, `RtcRtpEncodingParameters`*"]
    pub fn fec(&mut self, val: &RtcFecParameters) -> &mut Self {
        self.set_fec_shim(val);
        self
    }
    #[doc = "Change the `maxBitrate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`*"]
    pub fn max_bitrate(&mut self, val: u32) -> &mut Self {
        self.set_max_bitrate_shim(val);
        self
    }
    #[cfg(feature = "RtcPriorityType")]
    #[doc = "Change the `priority` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPriorityType`, `RtcRtpEncodingParameters`*"]
    pub fn priority(&mut self, val: RtcPriorityType) -> &mut Self {
        self.set_priority_shim(val);
        self
    }
    #[doc = "Change the `rid` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`*"]
    pub fn rid(&mut self, val: &str) -> &mut Self {
        self.set_rid_shim(val);
        self
    }
    #[cfg(feature = "RtcRtxParameters")]
    #[doc = "Change the `rtx` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`, `RtcRtxParameters`*"]
    pub fn rtx(&mut self, val: &RtcRtxParameters) -> &mut Self {
        self.set_rtx_shim(val);
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
        self.set_scalability_mode_shim(val);
        self
    }
    #[doc = "Change the `scaleResolutionDownBy` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`*"]
    pub fn scale_resolution_down_by(&mut self, val: f32) -> &mut Self {
        self.set_scale_resolution_down_by_shim(val);
        self
    }
    #[doc = "Change the `ssrc` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`*"]
    pub fn ssrc(&mut self, val: u32) -> &mut Self {
        self.set_ssrc_shim(val);
        self
    }
}
impl Default for RtcRtpEncodingParameters {
    fn default() -> Self {
        Self::new()
    }
}
