use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `MediaKeyMessageType` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `MediaKeyMessageType`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MediaKeyMessageType {
    LicenseRequest = "license-request",
    LicenseRenewal = "license-renewal",
    LicenseRelease = "license-release",
    IndividualizationRequest = "individualization-request",
}
