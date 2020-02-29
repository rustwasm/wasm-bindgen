use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `TcpReadyState` enum.
///
///*This API requires the following crate features to be activated: `TcpReadyState`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum TcpReadyState {
    Connecting = "connecting",
    Open = "open",
    Closing = "closing",
    Closed = "closed",
}
