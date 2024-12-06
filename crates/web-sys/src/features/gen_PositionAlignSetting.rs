#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen(no_export)]
#[doc = "The `PositionAlignSetting` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `PositionAlignSetting`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PositionAlignSetting {
    LineLeft = "line-left",
    Center = "center",
    LineRight = "line-right",
    Auto = "auto",
}
