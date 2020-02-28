use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `IdbRequestReadyState` enum.\n\n*This API requires the following crate features to be activated: `IdbRequestReadyState`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum IdbRequestReadyState {
    Pending = "pending",
    Done = "done",
}
