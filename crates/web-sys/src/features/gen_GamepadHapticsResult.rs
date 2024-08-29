#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `GamepadHapticsResult` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `GamepadHapticsResult`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GamepadHapticsResult {
    Complete = "complete",
    Preempted = "preempted",
}
