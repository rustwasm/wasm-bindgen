use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `RtcPriorityType` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcPriorityType`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum RtcPriorityType {
    VeryLow = "very-low",
    Low = "low",
    Medium = "medium",
    High = "high",
}
