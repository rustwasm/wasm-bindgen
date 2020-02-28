use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `PcImplIceConnectionState` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `PcImplIceConnectionState`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PcImplIceConnectionState {
    New = "new",
    Checking = "checking",
    Connected = "connected",
    Completed = "completed",
    Failed = "failed",
    Disconnected = "disconnected",
    Closed = "closed",
}
