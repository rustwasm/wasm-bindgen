use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `FlashClassification` enum.
///
///*This API requires the following crate features to be activated: `FlashClassification`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum FlashClassification {
    Unclassified = "unclassified",
    Unknown = "unknown",
    Allowed = "allowed",
    Denied = "denied",
}
