use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `FlashClassification` enum.\n\n*This API requires the following crate features to be activated: `FlashClassification`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum FlashClassification {
    Unclassified = "unclassified",
    Unknown = "unknown",
    Allowed = "allowed",
    Denied = "denied",
}
