use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `DistanceModelType` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `DistanceModelType`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum DistanceModelType {
    Linear = "linear",
    Inverse = "inverse",
    Exponential = "exponential",
}
