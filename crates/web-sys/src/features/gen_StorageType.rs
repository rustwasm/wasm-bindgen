use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `StorageType` enum.
///
///*This API requires the following crate features to be activated: `StorageType`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum StorageType {
    Persistent = "persistent",
    Temporary = "temporary",
    Default = "default",
}
