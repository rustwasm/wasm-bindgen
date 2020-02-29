use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `RecordingState` enum.
///
///*This API requires the following crate features to be activated: `RecordingState`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum RecordingState {
    Inactive = "inactive",
    Recording = "recording",
    Paused = "paused",
}
