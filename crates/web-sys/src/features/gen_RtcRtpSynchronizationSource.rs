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
    #[wasm_bindgen(method, setter = "audioLevel")]
    fn audio_level_shim(this: &RtcRtpSynchronizationSource, val: f64);
    #[wasm_bindgen(method, setter = "source")]
    fn source_shim(this: &RtcRtpSynchronizationSource, val: u32);
    #[wasm_bindgen(method, setter = "timestamp")]
    fn timestamp_shim(this: &RtcRtpSynchronizationSource, val: f64);
    #[wasm_bindgen(method, setter = "voiceActivityFlag")]
    fn voice_activity_flag_shim(this: &RtcRtpSynchronizationSource, val: Option<bool>);
}
impl RtcRtpSynchronizationSource {
    #[doc = "Construct a new `RtcRtpSynchronizationSource`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSynchronizationSource`*"]
    pub fn new(source: u32, timestamp: f64) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.source(source);
        ret.timestamp(timestamp);
        ret
    }
    #[doc = "Change the `audioLevel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSynchronizationSource`*"]
    pub fn audio_level(&mut self, val: f64) -> &mut Self {
        self.audio_level_shim(val);
        self
    }
    #[doc = "Change the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSynchronizationSource`*"]
    pub fn source(&mut self, val: u32) -> &mut Self {
        self.source_shim(val);
        self
    }
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSynchronizationSource`*"]
    pub fn timestamp(&mut self, val: f64) -> &mut Self {
        self.timestamp_shim(val);
        self
    }
    #[doc = "Change the `voiceActivityFlag` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSynchronizationSource`*"]
    pub fn voice_activity_flag(&mut self, val: Option<bool>) -> &mut Self {
        self.voice_activity_flag_shim(val);
        self
    }
}
