#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen(skip_typescript)]
#[doc = "The `PresentationConnectionClosedReason` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `PresentationConnectionClosedReason`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PresentationConnectionClosedReason {
    Error = "error",
    Closed = "closed",
    Wentaway = "wentaway",
}
