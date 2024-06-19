#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCIceComponentStats)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcIceComponentStats` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
    pub type RtcIceComponentStats;
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &RtcIceComponentStats) -> Option<String>;
    #[wasm_bindgen(method, setter = "id")]
    fn set_id(this: &RtcIceComponentStats, val: &str);
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
    #[wasm_bindgen(method, getter = "timestamp")]
    pub fn get_timestamp(this: &RtcIceComponentStats) -> Option<f64>;
    #[wasm_bindgen(method, setter = "timestamp")]
    fn set_timestamp(this: &RtcIceComponentStats, val: f64);
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`, `RtcStatsType`*"]
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &RtcIceComponentStats) -> Option<RtcStatsType>;
    #[cfg(feature = "RtcStatsType")]
    #[wasm_bindgen(method, setter = "type")]
    fn set_type(this: &RtcIceComponentStats, val: RtcStatsType);
    #[doc = "Get the `activeConnection` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
    #[wasm_bindgen(method, getter = "activeConnection")]
    pub fn get_active_connection(this: &RtcIceComponentStats) -> Option<bool>;
    #[wasm_bindgen(method, setter = "activeConnection")]
    fn set_active_connection(this: &RtcIceComponentStats, val: bool);
    #[doc = "Get the `bytesReceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
    #[wasm_bindgen(method, getter = "bytesReceived")]
    pub fn get_bytes_received(this: &RtcIceComponentStats) -> Option<u32>;
    #[wasm_bindgen(method, setter = "bytesReceived")]
    fn set_bytes_received(this: &RtcIceComponentStats, val: u32);
    #[doc = "Get the `bytesSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
    #[wasm_bindgen(method, getter = "bytesSent")]
    pub fn get_bytes_sent(this: &RtcIceComponentStats) -> Option<u32>;
    #[wasm_bindgen(method, setter = "bytesSent")]
    fn set_bytes_sent(this: &RtcIceComponentStats, val: u32);
    #[doc = "Get the `component` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
    #[wasm_bindgen(method, getter = "component")]
    pub fn get_component(this: &RtcIceComponentStats) -> Option<i32>;
    #[wasm_bindgen(method, setter = "component")]
    fn set_component(this: &RtcIceComponentStats, val: i32);
    #[doc = "Get the `transportId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
    #[wasm_bindgen(method, getter = "transportId")]
    pub fn get_transport_id(this: &RtcIceComponentStats) -> Option<String>;
    #[wasm_bindgen(method, setter = "transportId")]
    fn set_transport_id(this: &RtcIceComponentStats, val: &str);
}
impl RtcIceComponentStats {
    #[doc = "Construct a new `RtcIceComponentStats`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
    pub fn id(&mut self, val: &str) -> &mut Self {
        self.set_id(val);
        self
    }
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
    pub fn timestamp(&mut self, val: f64) -> &mut Self {
        self.set_timestamp(val);
        self
    }
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`, `RtcStatsType`*"]
    pub fn type_(&mut self, val: RtcStatsType) -> &mut Self {
        self.set_type(val);
        self
    }
    #[doc = "Change the `activeConnection` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
    pub fn active_connection(&mut self, val: bool) -> &mut Self {
        self.set_active_connection(val);
        self
    }
    #[doc = "Change the `bytesReceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
    pub fn bytes_received(&mut self, val: u32) -> &mut Self {
        self.set_bytes_received(val);
        self
    }
    #[doc = "Change the `bytesSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
    pub fn bytes_sent(&mut self, val: u32) -> &mut Self {
        self.set_bytes_sent(val);
        self
    }
    #[doc = "Change the `component` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
    pub fn component(&mut self, val: i32) -> &mut Self {
        self.set_component(val);
        self
    }
    #[doc = "Change the `transportId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
    pub fn transport_id(&mut self, val: &str) -> &mut Self {
        self.set_transport_id(val);
        self
    }
}
impl Default for RtcIceComponentStats {
    fn default() -> Self {
        Self::new()
    }
}
