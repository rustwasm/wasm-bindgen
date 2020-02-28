use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `GridTrackState` enum.\n\n*This API requires the following crate features to be activated: `GridTrackState`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum GridTrackState {
    Static = "static",
    Repeat = "repeat",
    Removed = "removed",
}
