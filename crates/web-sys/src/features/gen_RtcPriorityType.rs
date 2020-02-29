use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `RtcPriorityType` enum.
///
///*This API requires the following crate features to be activated: `RtcPriorityType`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum RtcPriorityType {
    VeryLow = "very-low",
    Low = "low",
    Medium = "medium",
    High = "high",
}
