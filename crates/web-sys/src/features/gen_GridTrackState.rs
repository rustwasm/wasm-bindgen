use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `GridTrackState` enum.
///
///*This API requires the following crate features to be activated: `GridTrackState`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum GridTrackState {
    Static = "static",
    Repeat = "repeat",
    Removed = "removed",
}
