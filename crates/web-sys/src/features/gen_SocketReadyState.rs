use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `SocketReadyState` enum.
///
///*This API requires the following crate features to be activated: `SocketReadyState`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum SocketReadyState {
    Opening = "opening",
    Open = "open",
    Closing = "closing",
    Closed = "closed",
    Halfclosed = "halfclosed",
}
