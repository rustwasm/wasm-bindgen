#![allow(unused_imports)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `WebTransportCongestionControl` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `WebTransportCongestionControl`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebTransportCongestionControl {
    Default = "default",
    Throughput = "throughput",
    LowLatency = "low-latency",
}
