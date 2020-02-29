use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `MediaKeyMessageType` enum.
///
///*This API requires the following crate features to be activated: `MediaKeyMessageType`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum MediaKeyMessageType {
    LicenseRequest = "license-request",
    LicenseRenewal = "license-renewal",
    LicenseRelease = "license-release",
    IndividualizationRequest = "individualization-request",
}
