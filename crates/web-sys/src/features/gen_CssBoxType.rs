use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `CssBoxType` enum.
///
///*This API requires the following crate features to be activated: `CssBoxType`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum CssBoxType {
    Margin = "margin",
    Border = "border",
    Padding = "padding",
    Content = "content",
}
