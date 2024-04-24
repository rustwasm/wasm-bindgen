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
    #[wasm_bindgen(method, getter = "id")]
    fn id_shim(this: &RtcIceComponentStats) -> &str;
    #[wasm_bindgen(method, setter = "id")]
    fn set_id_shim(this: &RtcIceComponentStats, val: &str);
    #[wasm_bindgen(method, getter = "timestamp")]
    fn timestamp_shim(this: &RtcIceComponentStats) -> f64;
    #[wasm_bindgen(method, setter = "timestamp")]
    fn set_timestamp_shim(this: &RtcIceComponentStats, val: f64);
    #[cfg(feature = "RtcStatsType")]
    #[wasm_bindgen(method, getter = "type")]
    fn type__shim(this: &RtcIceComponentStats) -> RtcStatsType;
    #[cfg(feature = "RtcStatsType")]
    #[wasm_bindgen(method, setter = "type")]
    fn set_type__shim(this: &RtcIceComponentStats, val: RtcStatsType);
    #[wasm_bindgen(method, getter = "activeConnection")]
    fn active_connection_shim(this: &RtcIceComponentStats) -> bool;
    #[wasm_bindgen(method, setter = "activeConnection")]
    fn set_active_connection_shim(this: &RtcIceComponentStats, val: bool);
    #[wasm_bindgen(method, getter = "bytesReceived")]
    fn bytes_received_shim(this: &RtcIceComponentStats) -> u32;
    #[wasm_bindgen(method, setter = "bytesReceived")]
    fn set_bytes_received_shim(this: &RtcIceComponentStats, val: u32);
    #[wasm_bindgen(method, getter = "bytesSent")]
    fn bytes_sent_shim(this: &RtcIceComponentStats) -> u32;
    #[wasm_bindgen(method, setter = "bytesSent")]
    fn set_bytes_sent_shim(this: &RtcIceComponentStats, val: u32);
    #[wasm_bindgen(method, getter = "component")]
    fn component_shim(this: &RtcIceComponentStats) -> i32;
    #[wasm_bindgen(method, setter = "component")]
    fn set_component_shim(this: &RtcIceComponentStats, val: i32);
    #[wasm_bindgen(method, getter = "transportId")]
    fn transport_id_shim(this: &RtcIceComponentStats) -> &str;
    #[wasm_bindgen(method, setter = "transportId")]
    fn set_transport_id_shim(this: &RtcIceComponentStats, val: &str);
}
#[doc = "The trait to access properties on the `RtcIceComponentStats` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
pub trait RtcIceComponentStatsGetters {
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
    fn id(&self) -> &str;
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
    fn timestamp(&self) -> f64;
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`, `RtcStatsType`*"]
    fn type_(&self) -> RtcStatsType;
    #[doc = "Get the `activeConnection` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
    fn active_connection(&self) -> bool;
    #[doc = "Get the `bytesReceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
    fn bytes_received(&self) -> u32;
    #[doc = "Get the `bytesSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
    fn bytes_sent(&self) -> u32;
    #[doc = "Get the `component` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
    fn component(&self) -> i32;
    #[doc = "Get the `transportId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
    fn transport_id(&self) -> &str;
}
impl RtcIceComponentStatsGetters for RtcIceComponentStats {
    fn id(&self) -> &str {
        self.id_shim()
    }
    fn timestamp(&self) -> f64 {
        self.timestamp_shim()
    }
    #[cfg(feature = "RtcStatsType")]
    fn type_(&self) -> RtcStatsType {
        self.type__shim()
    }
    fn active_connection(&self) -> bool {
        self.active_connection_shim()
    }
    fn bytes_received(&self) -> u32 {
        self.bytes_received_shim()
    }
    fn bytes_sent(&self) -> u32 {
        self.bytes_sent_shim()
    }
    fn component(&self) -> i32 {
        self.component_shim()
    }
    fn transport_id(&self) -> &str {
        self.transport_id_shim()
    }
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
        self.set_id_shim(val);
        self
    }
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
    pub fn timestamp(&mut self, val: f64) -> &mut Self {
        self.set_timestamp_shim(val);
        self
    }
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`, `RtcStatsType`*"]
    pub fn type_(&mut self, val: RtcStatsType) -> &mut Self {
        self.set_type__shim(val);
        self
    }
    #[doc = "Change the `activeConnection` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
    pub fn active_connection(&mut self, val: bool) -> &mut Self {
        self.set_active_connection_shim(val);
        self
    }
    #[doc = "Change the `bytesReceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
    pub fn bytes_received(&mut self, val: u32) -> &mut Self {
        self.set_bytes_received_shim(val);
        self
    }
    #[doc = "Change the `bytesSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
    pub fn bytes_sent(&mut self, val: u32) -> &mut Self {
        self.set_bytes_sent_shim(val);
        self
    }
    #[doc = "Change the `component` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
    pub fn component(&mut self, val: i32) -> &mut Self {
        self.set_component_shim(val);
        self
    }
    #[doc = "Change the `transportId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
    pub fn transport_id(&mut self, val: &str) -> &mut Self {
        self.set_transport_id_shim(val);
        self
    }
}
impl Default for RtcIceComponentStats {
    fn default() -> Self {
        Self::new()
    }
}
