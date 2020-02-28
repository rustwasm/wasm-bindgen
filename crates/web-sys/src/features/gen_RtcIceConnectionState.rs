use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `RtcIceConnectionState` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcIceConnectionState`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum RtcIceConnectionState {
    New = "new",
    Checking = "checking",
    Connected = "connected",
    Completed = "completed",
    Failed = "failed",
    Disconnected = "disconnected",
    Closed = "closed",
}
