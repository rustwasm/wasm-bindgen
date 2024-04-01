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
    #[wasm_bindgen(method, setter = "id")]
    fn id_shim(this: &RtcMediaStreamStats, val: &str);
    #[wasm_bindgen(method, setter = "timestamp")]
    fn timestamp_shim(this: &RtcMediaStreamStats, val: f64);
    #[cfg(feature = "RtcStatsType")]
    #[wasm_bindgen(method, setter = "type")]
    fn type__shim(this: &RtcMediaStreamStats, val: RtcStatsType);
    #[wasm_bindgen(method, setter = "streamIdentifier")]
    fn stream_identifier_shim(this: &RtcMediaStreamStats, val: &str);
    #[wasm_bindgen(method, setter = "trackIds")]
    fn track_ids_shim(this: &RtcMediaStreamStats, val: &::wasm_bindgen::JsValue);
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
        self.id_shim(val);
        self
    }
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamStats`*"]
    pub fn timestamp(&mut self, val: f64) -> &mut Self {
        self.timestamp_shim(val);
        self
    }
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamStats`, `RtcStatsType`*"]
    pub fn type_(&mut self, val: RtcStatsType) -> &mut Self {
        self.type__shim(val);
        self
    }
    #[doc = "Change the `streamIdentifier` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamStats`*"]
    pub fn stream_identifier(&mut self, val: &str) -> &mut Self {
        self.stream_identifier_shim(val);
        self
    }
    #[doc = "Change the `trackIds` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamStats`*"]
    pub fn track_ids(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.track_ids_shim(val);
        self
    }
}
impl Default for RtcMediaStreamStats {
    fn default() -> Self {
        Self::new()
    }
}
