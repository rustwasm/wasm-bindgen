use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `MediaKeysRequirement` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `MediaKeysRequirement`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MediaKeysRequirement {
    Required = "required",
    Optional = "optional",
    NotAllowed = "not-allowed",
}
