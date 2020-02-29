use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `UserVerificationRequirement` enum.
///
///*This API requires the following crate features to be activated: `UserVerificationRequirement`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum UserVerificationRequirement {
    Required = "required",
    Preferred = "preferred",
    Discouraged = "discouraged",
}
