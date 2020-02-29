use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `GamepadHand` enum.
///
///*This API requires the following crate features to be activated: `GamepadHand`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum GamepadHand {
    None = "",
    Left = "left",
    Right = "right",
}
