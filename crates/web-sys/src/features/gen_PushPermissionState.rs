use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `PushPermissionState` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `PushPermissionState`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PushPermissionState {
    Granted = "granted",
    Denied = "denied",
    Prompt = "prompt",
}
