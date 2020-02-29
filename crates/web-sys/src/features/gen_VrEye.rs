use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `VrEye` enum.
///
///*This API requires the following crate features to be activated: `VrEye`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum VrEye {
    Left = "left",
    Right = "right",
}
