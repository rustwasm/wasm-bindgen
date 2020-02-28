use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `RecordingState` enum.\n\n*This API requires the following crate features to be activated: `RecordingState`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum RecordingState {
    Inactive = "inactive",
    Recording = "recording",
    Paused = "paused",
}
