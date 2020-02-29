use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `MediaKeysRequirement` enum.
///
///*This API requires the following crate features to be activated: `MediaKeysRequirement`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum MediaKeysRequirement {
    Required = "required",
    Optional = "optional",
    NotAllowed = "not-allowed",
}
