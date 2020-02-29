use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `MediaSourceReadyState` enum.
///
///*This API requires the following crate features to be activated: `MediaSourceReadyState`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum MediaSourceReadyState {
    Closed = "closed",
    Open = "open",
    Ended = "ended",
}
