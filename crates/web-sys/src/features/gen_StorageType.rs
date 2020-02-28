use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `StorageType` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `StorageType`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum StorageType {
    Persistent = "persistent",
    Temporary = "temporary",
    Default = "default",
}
