use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `IdbRequestReadyState` enum.
///
///*This API requires the following crate features to be activated: `IdbRequestReadyState`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum IdbRequestReadyState {
    Pending = "pending",
    Done = "done",
}
