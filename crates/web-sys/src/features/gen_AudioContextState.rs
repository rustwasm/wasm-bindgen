use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `AudioContextState` enum.\n\n*This API requires the following crate features to be activated: `AudioContextState`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum AudioContextState {
    Suspended = "suspended",
    Running = "running",
    Closed = "closed",
}
