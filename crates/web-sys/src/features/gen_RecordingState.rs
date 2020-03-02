#![allow(unused_imports)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `RecordingState` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RecordingState`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordingState {
    Inactive = "inactive",
    Recording = "recording",
    Paused = "paused",
}
