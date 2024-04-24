#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = WebTransportDatagramStats)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebTransportDatagramStats` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportDatagramStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type WebTransportDatagramStats;
    #[wasm_bindgen(method, getter = "droppedIncoming")]
    fn dropped_incoming_shim(this: &WebTransportDatagramStats) -> f64;
    #[wasm_bindgen(method, setter = "droppedIncoming")]
    fn set_dropped_incoming_shim(this: &WebTransportDatagramStats, val: f64);
    #[wasm_bindgen(method, getter = "expiredOutgoing")]
    fn expired_outgoing_shim(this: &WebTransportDatagramStats) -> f64;
    #[wasm_bindgen(method, setter = "expiredOutgoing")]
    fn set_expired_outgoing_shim(this: &WebTransportDatagramStats, val: f64);
    #[wasm_bindgen(method, getter = "lostOutgoing")]
    fn lost_outgoing_shim(this: &WebTransportDatagramStats) -> f64;
    #[wasm_bindgen(method, setter = "lostOutgoing")]
    fn set_lost_outgoing_shim(this: &WebTransportDatagramStats, val: f64);
    #[wasm_bindgen(method, getter = "timestamp")]
    fn timestamp_shim(this: &WebTransportDatagramStats) -> f64;
    #[wasm_bindgen(method, setter = "timestamp")]
    fn set_timestamp_shim(this: &WebTransportDatagramStats, val: f64);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `WebTransportDatagramStats` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `WebTransportDatagramStats`*"]
pub trait WebTransportDatagramStatsGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `droppedIncoming` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportDatagramStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn dropped_incoming(&self) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `expiredOutgoing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportDatagramStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn expired_outgoing(&self) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `lostOutgoing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportDatagramStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn lost_outgoing(&self) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportDatagramStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn timestamp(&self) -> f64;
}
#[cfg(web_sys_unstable_apis)]
impl WebTransportDatagramStatsGetters for WebTransportDatagramStats {
    #[cfg(web_sys_unstable_apis)]
    fn dropped_incoming(&self) -> f64 {
        self.dropped_incoming_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn expired_outgoing(&self) -> f64 {
        self.expired_outgoing_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn lost_outgoing(&self) -> f64 {
        self.lost_outgoing_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn timestamp(&self) -> f64 {
        self.timestamp_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl WebTransportDatagramStats {
    #[doc = "Construct a new `WebTransportDatagramStats`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportDatagramStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `droppedIncoming` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportDatagramStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn dropped_incoming(&mut self, val: f64) -> &mut Self {
        self.set_dropped_incoming_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `expiredOutgoing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportDatagramStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn expired_outgoing(&mut self, val: f64) -> &mut Self {
        self.set_expired_outgoing_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `lostOutgoing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportDatagramStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn lost_outgoing(&mut self, val: f64) -> &mut Self {
        self.set_lost_outgoing_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportDatagramStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn timestamp(&mut self, val: f64) -> &mut Self {
        self.set_timestamp_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for WebTransportDatagramStats {
    fn default() -> Self {
        Self::new()
    }
}
