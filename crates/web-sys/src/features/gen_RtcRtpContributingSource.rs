#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCRtpContributingSource)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcRtpContributingSource` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpContributingSource`*"]
    pub type RtcRtpContributingSource;
    #[wasm_bindgen(method, getter = "audioLevel")]
    fn audio_level_shim(this: &RtcRtpContributingSource) -> f64;
    #[wasm_bindgen(method, setter = "audioLevel")]
    fn set_audio_level_shim(this: &RtcRtpContributingSource, val: f64);
    #[wasm_bindgen(method, getter = "source")]
    fn source_shim(this: &RtcRtpContributingSource) -> u32;
    #[wasm_bindgen(method, setter = "source")]
    fn set_source_shim(this: &RtcRtpContributingSource, val: u32);
    #[wasm_bindgen(method, getter = "timestamp")]
    fn timestamp_shim(this: &RtcRtpContributingSource) -> f64;
    #[wasm_bindgen(method, setter = "timestamp")]
    fn set_timestamp_shim(this: &RtcRtpContributingSource, val: f64);
}
#[doc = "The trait to access properties on the `RtcRtpContributingSource` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcRtpContributingSource`*"]
pub trait RtcRtpContributingSourceGetters {
    #[doc = "Get the `audioLevel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpContributingSource`*"]
    fn audio_level(&self) -> f64;
    #[doc = "Get the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpContributingSource`*"]
    fn source(&self) -> u32;
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpContributingSource`*"]
    fn timestamp(&self) -> f64;
}
impl RtcRtpContributingSourceGetters for RtcRtpContributingSource {
    fn audio_level(&self) -> f64 {
        self.audio_level_shim()
    }
    fn source(&self) -> u32 {
        self.source_shim()
    }
    fn timestamp(&self) -> f64 {
        self.timestamp_shim()
    }
}
impl RtcRtpContributingSource {
    #[doc = "Construct a new `RtcRtpContributingSource`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpContributingSource`*"]
    pub fn new(source: u32, timestamp: f64) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        Self::source(&mut ret, source);
        Self::timestamp(&mut ret, timestamp);
        ret
    }
    #[doc = "Change the `audioLevel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpContributingSource`*"]
    pub fn audio_level(&mut self, val: f64) -> &mut Self {
        self.set_audio_level_shim(val);
        self
    }
    #[doc = "Change the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpContributingSource`*"]
    pub fn source(&mut self, val: u32) -> &mut Self {
        self.set_source_shim(val);
        self
    }
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpContributingSource`*"]
    pub fn timestamp(&mut self, val: f64) -> &mut Self {
        self.set_timestamp_shim(val);
        self
    }
}
