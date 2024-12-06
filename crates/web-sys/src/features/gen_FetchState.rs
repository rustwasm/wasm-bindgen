#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen(no_export)]
#[doc = "The `FetchState` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `FetchState`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FetchState {
    Requesting = "requesting",
    Responding = "responding",
    Aborted = "aborted",
    Errored = "errored",
    Complete = "complete",
}
