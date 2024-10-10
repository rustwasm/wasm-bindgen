#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen(skip_typescript)]
#[doc = "The `IdbRequestReadyState` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `IdbRequestReadyState`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdbRequestReadyState {
    Pending = "pending",
    Done = "done",
}
