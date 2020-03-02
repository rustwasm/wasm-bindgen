use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `RtcLifecycleEvent` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcLifecycleEvent`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RtcLifecycleEvent {
    Initialized = "initialized",
    Icegatheringstatechange = "icegatheringstatechange",
    Iceconnectionstatechange = "iceconnectionstatechange",
}
