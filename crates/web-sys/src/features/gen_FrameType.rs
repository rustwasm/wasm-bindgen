#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen(no_export)]
#[doc = "The `FrameType` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `FrameType`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameType {
    Auxiliary = "auxiliary",
    TopLevel = "top-level",
    Nested = "nested",
    None = "none",
}
