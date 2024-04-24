#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCRtpSynchronizationSource)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcRtpSynchronizationSource` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSynchronizationSource`*"]
    pub type RtcRtpSynchronizationSource;
    #[wasm_bindgen(method, getter = "audioLevel")]
    fn audio_level_shim(this: &RtcRtpSynchronizationSource) -> f64;
    #[wasm_bindgen(method, setter = "audioLevel")]
    fn set_audio_level_shim(this: &RtcRtpSynchronizationSource, val: f64);
    #[wasm_bindgen(method, getter = "source")]
    fn source_shim(this: &RtcRtpSynchronizationSource) -> u32;
    #[wasm_bindgen(method, setter = "source")]
    fn set_source_shim(this: &RtcRtpSynchronizationSource, val: u32);
    #[wasm_bindgen(method, getter = "timestamp")]
    fn timestamp_shim(this: &RtcRtpSynchronizationSource) -> f64;
    #[wasm_bindgen(method, setter = "timestamp")]
    fn set_timestamp_shim(this: &RtcRtpSynchronizationSource, val: f64);
    #[wasm_bindgen(method, getter = "voiceActivityFlag")]
    fn voice_activity_flag_shim(this: &RtcRtpSynchronizationSource) -> Option<bool>;
    #[wasm_bindgen(method, setter = "voiceActivityFlag")]
    fn set_voice_activity_flag_shim(this: &RtcRtpSynchronizationSource, val: Option<bool>);
}
#[doc = "The trait to access properties on the `RtcRtpSynchronizationSource` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcRtpSynchronizationSource`*"]
pub trait RtcRtpSynchronizationSourceGetters {
    #[doc = "Get the `audioLevel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSynchronizationSource`*"]
    fn audio_level(&self) -> f64;
    #[doc = "Get the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSynchronizationSource`*"]
    fn source(&self) -> u32;
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSynchronizationSource`*"]
    fn timestamp(&self) -> f64;
    #[doc = "Get the `voiceActivityFlag` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSynchronizationSource`*"]
    fn voice_activity_flag(&self) -> Option<bool>;
}
impl RtcRtpSynchronizationSourceGetters for RtcRtpSynchronizationSource {
    fn audio_level(&self) -> f64 {
        self.audio_level_shim()
    }
    fn source(&self) -> u32 {
        self.source_shim()
    }
    fn timestamp(&self) -> f64 {
        self.timestamp_shim()
    }
    fn voice_activity_flag(&self) -> Option<bool> {
        self.voice_activity_flag_shim()
    }
}
impl RtcRtpSynchronizationSource {
    #[doc = "Construct a new `RtcRtpSynchronizationSource`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSynchronizationSource`*"]
    pub fn new(source: u32, timestamp: f64) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        Self::source(&mut ret, source);
        Self::timestamp(&mut ret, timestamp);
        ret
    }
    #[doc = "Change the `audioLevel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSynchronizationSource`*"]
    pub fn audio_level(&mut self, val: f64) -> &mut Self {
        self.set_audio_level_shim(val);
        self
    }
    #[doc = "Change the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSynchronizationSource`*"]
    pub fn source(&mut self, val: u32) -> &mut Self {
        self.set_source_shim(val);
        self
    }
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSynchronizationSource`*"]
    pub fn timestamp(&mut self, val: f64) -> &mut Self {
        self.set_timestamp_shim(val);
        self
    }
    #[doc = "Change the `voiceActivityFlag` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSynchronizationSource`*"]
    pub fn voice_activity_flag(&mut self, val: Option<bool>) -> &mut Self {
        self.set_voice_activity_flag_shim(val);
        self
    }
}
