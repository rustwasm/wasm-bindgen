use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `PermissionState` enum.
///
///*This API requires the following crate features to be activated: `PermissionState`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum PermissionState {
    Granted = "granted",
    Denied = "denied",
    Prompt = "prompt",
}
