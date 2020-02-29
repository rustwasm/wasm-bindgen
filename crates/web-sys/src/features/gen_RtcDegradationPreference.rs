use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `RtcDegradationPreference` enum.
///
///*This API requires the following crate features to be activated: `RtcDegradationPreference`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum RtcDegradationPreference {
    MaintainFramerate = "maintain-framerate",
    MaintainResolution = "maintain-resolution",
    Balanced = "balanced",
}
