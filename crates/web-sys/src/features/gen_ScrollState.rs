use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `ScrollState` enum.
///
///*This API requires the following crate features to be activated: `ScrollState`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum ScrollState {
    Started = "started",
    Stopped = "stopped",
}
