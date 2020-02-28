use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `MediaStreamTrackState` enum.\n\n*This API requires the following crate features to be activated: `MediaStreamTrackState`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MediaStreamTrackState {
    Live = "live",
    Ended = "ended",
}
