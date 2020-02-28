use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `RtcDegradationPreference` enum.\n\n*This API requires the following crate features to be activated: `RtcDegradationPreference`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum RtcDegradationPreference {
    MaintainFramerate = "maintain-framerate",
    MaintainResolution = "maintain-resolution",
    Balanced = "balanced",
}
