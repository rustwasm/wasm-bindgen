use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `DirectionSetting` enum.
///
///*This API requires the following crate features to be activated: `DirectionSetting`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum DirectionSetting {
    None = "",
    Rl = "rl",
    Lr = "lr",
}
