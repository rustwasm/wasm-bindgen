use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `PermissionName` enum.\n\n*This API requires the following crate features to be activated: `PermissionName`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PermissionName {
    Geolocation = "geolocation",
    Notifications = "notifications",
    Push = "push",
    PersistentStorage = "persistent-storage",
}
