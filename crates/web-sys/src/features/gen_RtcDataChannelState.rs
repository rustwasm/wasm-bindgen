use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `RtcDataChannelState` enum.\n\n*This API requires the following crate features to be activated: `RtcDataChannelState`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum RtcDataChannelState {
    Connecting = "connecting",
    Open = "open",
    Closing = "closing",
    Closed = "closed",
}
