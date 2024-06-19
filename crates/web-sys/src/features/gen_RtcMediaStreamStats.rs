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
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamStats`*"]
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &RtcMediaStreamStats) -> Option<String>;
    #[wasm_bindgen(method, setter = "id")]
    fn set_id(this: &RtcMediaStreamStats, val: &str);
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamStats`*"]
    #[wasm_bindgen(method, getter = "timestamp")]
    pub fn get_timestamp(this: &RtcMediaStreamStats) -> Option<f64>;
    #[wasm_bindgen(method, setter = "timestamp")]
    fn set_timestamp(this: &RtcMediaStreamStats, val: f64);
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamStats`, `RtcStatsType`*"]
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &RtcMediaStreamStats) -> Option<RtcStatsType>;
    #[cfg(feature = "RtcStatsType")]
    #[wasm_bindgen(method, setter = "type")]
    fn set_type(this: &RtcMediaStreamStats, val: RtcStatsType);
    #[doc = "Get the `streamIdentifier` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamStats`*"]
    #[wasm_bindgen(method, getter = "streamIdentifier")]
    pub fn get_stream_identifier(this: &RtcMediaStreamStats) -> Option<String>;
    #[wasm_bindgen(method, setter = "streamIdentifier")]
    fn set_stream_identifier(this: &RtcMediaStreamStats, val: &str);
    #[doc = "Get the `trackIds` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamStats`*"]
    #[wasm_bindgen(method, getter = "trackIds")]
    pub fn get_track_ids(this: &RtcMediaStreamStats) -> Option<::js_sys::Array>;
    #[wasm_bindgen(method, setter = "trackIds")]
    fn set_track_ids(this: &RtcMediaStreamStats, val: &::wasm_bindgen::JsValue);
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
        self.set_id(val);
        self
    }
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamStats`*"]
    pub fn timestamp(&mut self, val: f64) -> &mut Self {
        self.set_timestamp(val);
        self
    }
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamStats`, `RtcStatsType`*"]
    pub fn type_(&mut self, val: RtcStatsType) -> &mut Self {
        self.set_type(val);
        self
    }
    #[doc = "Change the `streamIdentifier` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamStats`*"]
    pub fn stream_identifier(&mut self, val: &str) -> &mut Self {
        self.set_stream_identifier(val);
        self
    }
    #[doc = "Change the `trackIds` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamStats`*"]
    pub fn track_ids(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_track_ids(val);
        self
    }
}
impl Default for RtcMediaStreamStats {
    fn default() -> Self {
        Self::new()
    }
}
