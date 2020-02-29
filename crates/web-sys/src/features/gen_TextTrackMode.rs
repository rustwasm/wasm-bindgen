use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `TextTrackMode` enum.
///
///*This API requires the following crate features to be activated: `TextTrackMode`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum TextTrackMode {
    Disabled = "disabled",
    Hidden = "hidden",
    Showing = "showing",
}
