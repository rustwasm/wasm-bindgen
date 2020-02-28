use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `PermissionState` enum.\n\n*This API requires the following crate features to be activated: `PermissionState`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PermissionState {
    Granted = "granted",
    Denied = "denied",
    Prompt = "prompt",
}
