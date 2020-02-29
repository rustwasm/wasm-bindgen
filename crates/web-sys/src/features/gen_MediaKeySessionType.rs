use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `MediaKeySessionType` enum.
///
///*This API requires the following crate features to be activated: `MediaKeySessionType`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum MediaKeySessionType {
    Temporary = "temporary",
    PersistentLicense = "persistent-license",
}
