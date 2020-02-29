use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `DistanceModelType` enum.
///
///*This API requires the following crate features to be activated: `DistanceModelType`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum DistanceModelType {
    Linear = "linear",
    Inverse = "inverse",
    Exponential = "exponential",
}
