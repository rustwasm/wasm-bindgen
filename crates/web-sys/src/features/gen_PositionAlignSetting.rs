use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `PositionAlignSetting` enum.
///
///*This API requires the following crate features to be activated: `PositionAlignSetting`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum PositionAlignSetting {
    LineLeft = "line-left",
    Center = "center",
    LineRight = "line-right",
    Auto = "auto",
}
