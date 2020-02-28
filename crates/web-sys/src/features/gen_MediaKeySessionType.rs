use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `MediaKeySessionType` enum.\n\n*This API requires the following crate features to be activated: `MediaKeySessionType`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MediaKeySessionType {
    Temporary = "temporary",
    PersistentLicense = "persistent-license",
}
