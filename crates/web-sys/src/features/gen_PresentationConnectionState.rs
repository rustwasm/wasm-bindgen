use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `PresentationConnectionState` enum.\n\n*This API requires the following crate features to be activated: `PresentationConnectionState`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PresentationConnectionState {
    Connecting = "connecting",
    Connected = "connected",
    Closed = "closed",
    Terminated = "terminated",
}
