use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `PanningModelType` enum.
///
///*This API requires the following crate features to be activated: `PanningModelType`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum PanningModelType {
    Equalpower = "equalpower",
    Hrtf = "HRTF",
}
