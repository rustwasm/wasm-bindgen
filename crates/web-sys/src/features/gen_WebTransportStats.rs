#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = WebTransportStats)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebTransportStats` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type WebTransportStats;
    #[wasm_bindgen(method, getter = "bytesReceived")]
    fn bytes_received_shim(this: &WebTransportStats) -> f64;
    #[wasm_bindgen(method, setter = "bytesReceived")]
    fn set_bytes_received_shim(this: &WebTransportStats, val: f64);
    #[wasm_bindgen(method, getter = "bytesSent")]
    fn bytes_sent_shim(this: &WebTransportStats) -> f64;
    #[wasm_bindgen(method, setter = "bytesSent")]
    fn set_bytes_sent_shim(this: &WebTransportStats, val: f64);
    #[cfg(feature = "WebTransportDatagramStats")]
    #[wasm_bindgen(method, getter = "datagrams")]
    fn datagrams_shim(this: &WebTransportStats) -> &WebTransportDatagramStats;
    #[cfg(feature = "WebTransportDatagramStats")]
    #[wasm_bindgen(method, setter = "datagrams")]
    fn set_datagrams_shim(this: &WebTransportStats, val: &WebTransportDatagramStats);
    #[wasm_bindgen(method, getter = "minRtt")]
    fn min_rtt_shim(this: &WebTransportStats) -> f64;
    #[wasm_bindgen(method, setter = "minRtt")]
    fn set_min_rtt_shim(this: &WebTransportStats, val: f64);
    #[wasm_bindgen(method, getter = "numIncomingStreamsCreated")]
    fn num_incoming_streams_created_shim(this: &WebTransportStats) -> u32;
    #[wasm_bindgen(method, setter = "numIncomingStreamsCreated")]
    fn set_num_incoming_streams_created_shim(this: &WebTransportStats, val: u32);
    #[wasm_bindgen(method, getter = "numOutgoingStreamsCreated")]
    fn num_outgoing_streams_created_shim(this: &WebTransportStats) -> u32;
    #[wasm_bindgen(method, setter = "numOutgoingStreamsCreated")]
    fn set_num_outgoing_streams_created_shim(this: &WebTransportStats, val: u32);
    #[wasm_bindgen(method, getter = "packetsLost")]
    fn packets_lost_shim(this: &WebTransportStats) -> f64;
    #[wasm_bindgen(method, setter = "packetsLost")]
    fn set_packets_lost_shim(this: &WebTransportStats, val: f64);
    #[wasm_bindgen(method, getter = "packetsReceived")]
    fn packets_received_shim(this: &WebTransportStats) -> f64;
    #[wasm_bindgen(method, setter = "packetsReceived")]
    fn set_packets_received_shim(this: &WebTransportStats, val: f64);
    #[wasm_bindgen(method, getter = "packetsSent")]
    fn packets_sent_shim(this: &WebTransportStats) -> f64;
    #[wasm_bindgen(method, setter = "packetsSent")]
    fn set_packets_sent_shim(this: &WebTransportStats, val: f64);
    #[wasm_bindgen(method, getter = "rttVariation")]
    fn rtt_variation_shim(this: &WebTransportStats) -> f64;
    #[wasm_bindgen(method, setter = "rttVariation")]
    fn set_rtt_variation_shim(this: &WebTransportStats, val: f64);
    #[wasm_bindgen(method, getter = "smoothedRtt")]
    fn smoothed_rtt_shim(this: &WebTransportStats) -> f64;
    #[wasm_bindgen(method, setter = "smoothedRtt")]
    fn set_smoothed_rtt_shim(this: &WebTransportStats, val: f64);
    #[wasm_bindgen(method, getter = "timestamp")]
    fn timestamp_shim(this: &WebTransportStats) -> f64;
    #[wasm_bindgen(method, setter = "timestamp")]
    fn set_timestamp_shim(this: &WebTransportStats, val: f64);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `WebTransportStats` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `WebTransportStats`*"]
pub trait WebTransportStatsGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `bytesReceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn bytes_received(&self) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `bytesSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn bytes_sent(&self) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "WebTransportDatagramStats")]
    #[doc = "Get the `datagrams` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportDatagramStats`, `WebTransportStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn datagrams(&self) -> &WebTransportDatagramStats;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `minRtt` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn min_rtt(&self) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `numIncomingStreamsCreated` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn num_incoming_streams_created(&self) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `numOutgoingStreamsCreated` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn num_outgoing_streams_created(&self) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `packetsLost` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn packets_lost(&self) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `packetsReceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn packets_received(&self) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `packetsSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn packets_sent(&self) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `rttVariation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn rtt_variation(&self) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `smoothedRtt` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn smoothed_rtt(&self) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn timestamp(&self) -> f64;
}
#[cfg(web_sys_unstable_apis)]
impl WebTransportStatsGetters for WebTransportStats {
    #[cfg(web_sys_unstable_apis)]
    fn bytes_received(&self) -> f64 {
        self.bytes_received_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn bytes_sent(&self) -> f64 {
        self.bytes_sent_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "WebTransportDatagramStats")]
    fn datagrams(&self) -> &WebTransportDatagramStats {
        self.datagrams_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn min_rtt(&self) -> f64 {
        self.min_rtt_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn num_incoming_streams_created(&self) -> u32 {
        self.num_incoming_streams_created_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn num_outgoing_streams_created(&self) -> u32 {
        self.num_outgoing_streams_created_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn packets_lost(&self) -> f64 {
        self.packets_lost_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn packets_received(&self) -> f64 {
        self.packets_received_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn packets_sent(&self) -> f64 {
        self.packets_sent_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn rtt_variation(&self) -> f64 {
        self.rtt_variation_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn smoothed_rtt(&self) -> f64 {
        self.smoothed_rtt_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn timestamp(&self) -> f64 {
        self.timestamp_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl WebTransportStats {
    #[doc = "Construct a new `WebTransportStats`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `bytesReceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn bytes_received(&mut self, val: f64) -> &mut Self {
        self.set_bytes_received_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `bytesSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn bytes_sent(&mut self, val: f64) -> &mut Self {
        self.set_bytes_sent_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "WebTransportDatagramStats")]
    #[doc = "Change the `datagrams` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportDatagramStats`, `WebTransportStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn datagrams(&mut self, val: &WebTransportDatagramStats) -> &mut Self {
        self.set_datagrams_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `minRtt` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn min_rtt(&mut self, val: f64) -> &mut Self {
        self.set_min_rtt_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `numIncomingStreamsCreated` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn num_incoming_streams_created(&mut self, val: u32) -> &mut Self {
        self.set_num_incoming_streams_created_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `numOutgoingStreamsCreated` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn num_outgoing_streams_created(&mut self, val: u32) -> &mut Self {
        self.set_num_outgoing_streams_created_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `packetsLost` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn packets_lost(&mut self, val: f64) -> &mut Self {
        self.set_packets_lost_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `packetsReceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn packets_received(&mut self, val: f64) -> &mut Self {
        self.set_packets_received_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `packetsSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn packets_sent(&mut self, val: f64) -> &mut Self {
        self.set_packets_sent_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `rttVariation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn rtt_variation(&mut self, val: f64) -> &mut Self {
        self.set_rtt_variation_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `smoothedRtt` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn smoothed_rtt(&mut self, val: f64) -> &mut Self {
        self.set_smoothed_rtt_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn timestamp(&mut self, val: f64) -> &mut Self {
        self.set_timestamp_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for WebTransportStats {
    fn default() -> Self {
        Self::new()
    }
}
