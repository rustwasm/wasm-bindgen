#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen(no_export)]
#[doc = "The `CanvasWindingRule` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `CanvasWindingRule`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanvasWindingRule {
    Nonzero = "nonzero",
    Evenodd = "evenodd",
}
