use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `AudioContextState` enum.
///
///*This API requires the following crate features to be activated: `AudioContextState`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum AudioContextState {
    Suspended = "suspended",
    Running = "running",
    Closed = "closed",
}
