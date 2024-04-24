#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = WebTransportSendStreamStats)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebTransportSendStreamStats` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportSendStreamStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type WebTransportSendStreamStats;
    #[wasm_bindgen(method, getter = "bytesAcknowledged")]
    fn bytes_acknowledged_shim(this: &WebTransportSendStreamStats) -> f64;
    #[wasm_bindgen(method, setter = "bytesAcknowledged")]
    fn set_bytes_acknowledged_shim(this: &WebTransportSendStreamStats, val: f64);
    #[wasm_bindgen(method, getter = "bytesSent")]
    fn bytes_sent_shim(this: &WebTransportSendStreamStats) -> f64;
    #[wasm_bindgen(method, setter = "bytesSent")]
    fn set_bytes_sent_shim(this: &WebTransportSendStreamStats, val: f64);
    #[wasm_bindgen(method, getter = "bytesWritten")]
    fn bytes_written_shim(this: &WebTransportSendStreamStats) -> f64;
    #[wasm_bindgen(method, setter = "bytesWritten")]
    fn set_bytes_written_shim(this: &WebTransportSendStreamStats, val: f64);
    #[wasm_bindgen(method, getter = "timestamp")]
    fn timestamp_shim(this: &WebTransportSendStreamStats) -> f64;
    #[wasm_bindgen(method, setter = "timestamp")]
    fn set_timestamp_shim(this: &WebTransportSendStreamStats, val: f64);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `WebTransportSendStreamStats` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `WebTransportSendStreamStats`*"]
pub trait WebTransportSendStreamStatsGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `bytesAcknowledged` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportSendStreamStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn bytes_acknowledged(&self) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `bytesSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportSendStreamStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn bytes_sent(&self) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `bytesWritten` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportSendStreamStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn bytes_written(&self) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportSendStreamStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn timestamp(&self) -> f64;
}
#[cfg(web_sys_unstable_apis)]
impl WebTransportSendStreamStatsGetters for WebTransportSendStreamStats {
    #[cfg(web_sys_unstable_apis)]
    fn bytes_acknowledged(&self) -> f64 {
        self.bytes_acknowledged_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn bytes_sent(&self) -> f64 {
        self.bytes_sent_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn bytes_written(&self) -> f64 {
        self.bytes_written_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn timestamp(&self) -> f64 {
        self.timestamp_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl WebTransportSendStreamStats {
    #[doc = "Construct a new `WebTransportSendStreamStats`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportSendStreamStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `bytesAcknowledged` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportSendStreamStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn bytes_acknowledged(&mut self, val: f64) -> &mut Self {
        self.set_bytes_acknowledged_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `bytesSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportSendStreamStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn bytes_sent(&mut self, val: f64) -> &mut Self {
        self.set_bytes_sent_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `bytesWritten` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportSendStreamStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn bytes_written(&mut self, val: f64) -> &mut Self {
        self.set_bytes_written_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportSendStreamStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn timestamp(&mut self, val: f64) -> &mut Self {
        self.set_timestamp_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for WebTransportSendStreamStats {
    fn default() -> Self {
        Self::new()
    }
}
