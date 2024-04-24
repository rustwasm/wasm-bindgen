#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCMediaStreamStats)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcMediaStreamStats` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamStats`*"]
    pub type RtcMediaStreamStats;
    #[wasm_bindgen(method, getter = "id")]
    fn id_shim(this: &RtcMediaStreamStats) -> String;
    #[wasm_bindgen(method, setter = "id")]
    fn set_id_shim(this: &RtcMediaStreamStats, val: &str);
    #[wasm_bindgen(method, getter = "timestamp")]
    fn timestamp_shim(this: &RtcMediaStreamStats) -> f64;
    #[wasm_bindgen(method, setter = "timestamp")]
    fn set_timestamp_shim(this: &RtcMediaStreamStats, val: f64);
    #[cfg(feature = "RtcStatsType")]
    #[wasm_bindgen(method, getter = "type")]
    fn type__shim(this: &RtcMediaStreamStats) -> RtcStatsType;
    #[cfg(feature = "RtcStatsType")]
    #[wasm_bindgen(method, setter = "type")]
    fn set_type__shim(this: &RtcMediaStreamStats, val: RtcStatsType);
    #[wasm_bindgen(method, getter = "streamIdentifier")]
    fn stream_identifier_shim(this: &RtcMediaStreamStats) -> String;
    #[wasm_bindgen(method, setter = "streamIdentifier")]
    fn set_stream_identifier_shim(this: &RtcMediaStreamStats, val: &str);
    #[wasm_bindgen(method, getter = "trackIds")]
    fn track_ids_shim(this: &RtcMediaStreamStats) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "trackIds")]
    fn set_track_ids_shim(this: &RtcMediaStreamStats, val: &::wasm_bindgen::JsValue);
}
#[doc = "The trait to access properties on the `RtcMediaStreamStats` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamStats`*"]
pub trait RtcMediaStreamStatsGetters {
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamStats`*"]
    fn id(&self) -> String;
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamStats`*"]
    fn timestamp(&self) -> f64;
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamStats`, `RtcStatsType`*"]
    fn type_(&self) -> RtcStatsType;
    #[doc = "Get the `streamIdentifier` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamStats`*"]
    fn stream_identifier(&self) -> String;
    #[doc = "Get the `trackIds` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamStats`*"]
    fn track_ids(&self) -> ::js_sys::Array;
}
impl RtcMediaStreamStatsGetters for RtcMediaStreamStats {
    fn id(&self) -> String {
        self.id_shim()
    }
    fn timestamp(&self) -> f64 {
        self.timestamp_shim()
    }
    #[cfg(feature = "RtcStatsType")]
    fn type_(&self) -> RtcStatsType {
        self.type__shim()
    }
    fn stream_identifier(&self) -> String {
        self.stream_identifier_shim()
    }
    fn track_ids(&self) -> ::js_sys::Array {
        self.track_ids_shim()
    }
}
impl RtcMediaStreamStats {
    #[doc = "Construct a new `RtcMediaStreamStats`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamStats`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamStats`*"]
    pub fn id(&mut self, val: &str) -> &mut Self {
        self.set_id_shim(val);
        self
    }
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamStats`*"]
    pub fn timestamp(&mut self, val: f64) -> &mut Self {
        self.set_timestamp_shim(val);
        self
    }
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamStats`, `RtcStatsType`*"]
    pub fn type_(&mut self, val: RtcStatsType) -> &mut Self {
        self.set_type__shim(val);
        self
    }
    #[doc = "Change the `streamIdentifier` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamStats`*"]
    pub fn stream_identifier(&mut self, val: &str) -> &mut Self {
        self.set_stream_identifier_shim(val);
        self
    }
    #[doc = "Change the `trackIds` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamStats`*"]
    pub fn track_ids(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_track_ids_shim(val);
        self
    }
}
impl Default for RtcMediaStreamStats {
    fn default() -> Self {
        Self::new()
    }
}
