use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `RtcLifecycleEvent` enum.
///
///*This API requires the following crate features to be activated: `RtcLifecycleEvent`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum RtcLifecycleEvent {
    Initialized = "initialized",
    Icegatheringstatechange = "icegatheringstatechange",
    Iceconnectionstatechange = "iceconnectionstatechange",
}
