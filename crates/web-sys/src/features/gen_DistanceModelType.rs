#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen(no_export)]
#[doc = "The `DistanceModelType` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `DistanceModelType`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DistanceModelType {
    Linear = "linear",
    Inverse = "inverse",
    Exponential = "exponential",
}
