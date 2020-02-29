use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `CssStyleSheetParsingMode` enum.
///
///*This API requires the following crate features to be activated: `CssStyleSheetParsingMode`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum CssStyleSheetParsingMode {
    Author = "author",
    User = "user",
    Agent = "agent",
}
