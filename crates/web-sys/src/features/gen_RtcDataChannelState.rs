use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `RtcDataChannelState` enum.
///
///*This API requires the following crate features to be activated: `RtcDataChannelState`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum RtcDataChannelState {
    Connecting = "connecting",
    Open = "open",
    Closing = "closing",
    Closed = "closed",
}
