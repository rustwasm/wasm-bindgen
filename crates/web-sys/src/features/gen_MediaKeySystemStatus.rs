use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `MediaKeySystemStatus` enum.\n\n*This API requires the following crate features to be activated: `MediaKeySystemStatus`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MediaKeySystemStatus {
    Available = "available",
    ApiDisabled = "api-disabled",
    CdmDisabled = "cdm-disabled",
    CdmNotSupported = "cdm-not-supported",
    CdmNotInstalled = "cdm-not-installed",
    CdmCreated = "cdm-created",
}
