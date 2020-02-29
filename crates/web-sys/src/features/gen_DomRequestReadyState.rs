use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `DomRequestReadyState` enum.
///
///*This API requires the following crate features to be activated: `DomRequestReadyState`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum DomRequestReadyState {
    Pending = "pending",
    Done = "done",
}
