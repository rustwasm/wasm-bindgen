#![allow(unused_imports)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `CssBoxType` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `CssBoxType`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CssBoxType {
    Margin = "margin",
    Border = "border",
    Padding = "padding",
    Content = "content",
}
