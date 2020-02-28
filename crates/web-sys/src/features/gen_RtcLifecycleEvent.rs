use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `RtcLifecycleEvent` enum.\n\n*This API requires the following crate features to be activated: `RtcLifecycleEvent`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum RtcLifecycleEvent {
    Initialized = "initialized",
    Icegatheringstatechange = "icegatheringstatechange",
    Iceconnectionstatechange = "iceconnectionstatechange",
}
