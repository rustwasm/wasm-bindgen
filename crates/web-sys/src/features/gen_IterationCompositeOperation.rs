#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen(skip_typescript)]
#[doc = "The `IterationCompositeOperation` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `IterationCompositeOperation`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IterationCompositeOperation {
    Replace = "replace",
    Accumulate = "accumulate",
}
