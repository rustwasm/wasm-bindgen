use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `ChannelCountMode` enum.
///
///*This API requires the following crate features to be activated: `ChannelCountMode`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum ChannelCountMode {
    Max = "max",
    ClampedMax = "clamped-max",
    Explicit = "explicit",
}
