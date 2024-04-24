#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCRtpSourceEntry)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcRtpSourceEntry` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSourceEntry`*"]
    pub type RtcRtpSourceEntry;
    #[wasm_bindgen(method, getter = "audioLevel")]
    fn audio_level_shim(this: &RtcRtpSourceEntry) -> f64;
    #[wasm_bindgen(method, setter = "audioLevel")]
    fn set_audio_level_shim(this: &RtcRtpSourceEntry, val: f64);
    #[wasm_bindgen(method, getter = "source")]
    fn source_shim(this: &RtcRtpSourceEntry) -> u32;
    #[wasm_bindgen(method, setter = "source")]
    fn set_source_shim(this: &RtcRtpSourceEntry, val: u32);
    #[wasm_bindgen(method, getter = "timestamp")]
    fn timestamp_shim(this: &RtcRtpSourceEntry) -> f64;
    #[wasm_bindgen(method, setter = "timestamp")]
    fn set_timestamp_shim(this: &RtcRtpSourceEntry, val: f64);
    #[wasm_bindgen(method, getter = "voiceActivityFlag")]
    fn voice_activity_flag_shim(this: &RtcRtpSourceEntry) -> Option<bool>;
    #[wasm_bindgen(method, setter = "voiceActivityFlag")]
    fn set_voice_activity_flag_shim(this: &RtcRtpSourceEntry, val: Option<bool>);
    #[cfg(feature = "RtcRtpSourceEntryType")]
    #[wasm_bindgen(method, getter = "sourceType")]
    fn source_type_shim(this: &RtcRtpSourceEntry) -> RtcRtpSourceEntryType;
    #[cfg(feature = "RtcRtpSourceEntryType")]
    #[wasm_bindgen(method, setter = "sourceType")]
    fn set_source_type_shim(this: &RtcRtpSourceEntry, val: RtcRtpSourceEntryType);
}
#[doc = "The trait to access properties on the `RtcRtpSourceEntry` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcRtpSourceEntry`*"]
pub trait RtcRtpSourceEntryGetters {
    #[doc = "Get the `audioLevel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSourceEntry`*"]
    fn audio_level(&self) -> f64;
    #[doc = "Get the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSourceEntry`*"]
    fn source(&self) -> u32;
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSourceEntry`*"]
    fn timestamp(&self) -> f64;
    #[doc = "Get the `voiceActivityFlag` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSourceEntry`*"]
    fn voice_activity_flag(&self) -> Option<bool>;
    #[cfg(feature = "RtcRtpSourceEntryType")]
    #[doc = "Get the `sourceType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSourceEntry`, `RtcRtpSourceEntryType`*"]
    fn source_type(&self) -> RtcRtpSourceEntryType;
}
impl RtcRtpSourceEntryGetters for RtcRtpSourceEntry {
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
    #[cfg(feature = "RtcRtpSourceEntryType")]
    fn source_type(&self) -> RtcRtpSourceEntryType {
        self.source_type_shim()
    }
}
impl RtcRtpSourceEntry {
    #[cfg(feature = "RtcRtpSourceEntryType")]
    #[doc = "Construct a new `RtcRtpSourceEntry`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSourceEntry`, `RtcRtpSourceEntryType`*"]
    pub fn new(source: u32, timestamp: f64, source_type: RtcRtpSourceEntryType) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.source(source);
        ret.timestamp(timestamp);
        ret.source_type(source_type);
        ret
    }
    #[doc = "Change the `audioLevel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSourceEntry`*"]
    pub fn audio_level(&mut self, val: f64) -> &mut Self {
        self.set_audio_level_shim(val);
        self
    }
    #[doc = "Change the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSourceEntry`*"]
    pub fn source(&mut self, val: u32) -> &mut Self {
        self.set_source_shim(val);
        self
    }
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSourceEntry`*"]
    pub fn timestamp(&mut self, val: f64) -> &mut Self {
        self.set_timestamp_shim(val);
        self
    }
    #[doc = "Change the `voiceActivityFlag` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSourceEntry`*"]
    pub fn voice_activity_flag(&mut self, val: Option<bool>) -> &mut Self {
        self.set_voice_activity_flag_shim(val);
        self
    }
    #[cfg(feature = "RtcRtpSourceEntryType")]
    #[doc = "Change the `sourceType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSourceEntry`, `RtcRtpSourceEntryType`*"]
    pub fn source_type(&mut self, val: RtcRtpSourceEntryType) -> &mut Self {
        self.set_source_type_shim(val);
        self
    }
}
