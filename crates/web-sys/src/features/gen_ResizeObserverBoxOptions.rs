#![allow(unused_imports)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `ResizeObserverBoxOptions` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ResizeObserverBoxOptions`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResizeObserverBoxOptions {
    BorderBox = "border-box",
    ContentBox = "content-box",
    DevicePixelContentBox = "device-pixel-content-box",
}
