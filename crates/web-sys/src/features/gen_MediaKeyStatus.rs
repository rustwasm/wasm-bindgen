use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `MediaKeyStatus` enum.\n\n*This API requires the following crate features to be activated: `MediaKeyStatus`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MediaKeyStatus {
    Usable = "usable",
    Expired = "expired",
    Released = "released",
    OutputRestricted = "output-restricted",
    OutputDownscaled = "output-downscaled",
    StatusPending = "status-pending",
    InternalError = "internal-error",
}
