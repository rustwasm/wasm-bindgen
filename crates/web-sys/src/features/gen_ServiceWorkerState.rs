use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `ServiceWorkerState` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ServiceWorkerState`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ServiceWorkerState {
    Parsed = "parsed",
    Installing = "installing",
    Installed = "installed",
    Activating = "activating",
    Activated = "activated",
    Redundant = "redundant",
}
