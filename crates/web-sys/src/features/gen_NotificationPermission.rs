use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `NotificationPermission` enum.\n\n*This API requires the following crate features to be activated: `NotificationPermission`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum NotificationPermission {
    Default = "default",
    Denied = "denied",
    Granted = "granted",
}
