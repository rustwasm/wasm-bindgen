use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `ScrollSetting` enum.
///
///*This API requires the following crate features to be activated: `ScrollSetting`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum ScrollSetting {
    None = "",
    Up = "up",
}
