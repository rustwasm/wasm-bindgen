use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `TcpReadyState` enum.\n\n*This API requires the following crate features to be activated: `TcpReadyState`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TcpReadyState {
    Connecting = "connecting",
    Open = "open",
    Closing = "closing",
    Closed = "closed",
}
