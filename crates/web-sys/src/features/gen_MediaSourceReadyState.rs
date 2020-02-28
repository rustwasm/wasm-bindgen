use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `MediaSourceReadyState` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `MediaSourceReadyState`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MediaSourceReadyState {
    Closed = "closed",
    Open = "open",
    Ended = "ended",
}
