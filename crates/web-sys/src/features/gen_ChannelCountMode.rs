use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `ChannelCountMode` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ChannelCountMode {
    Max = "max",
    ClampedMax = "clamped-max",
    Explicit = "explicit",
}
