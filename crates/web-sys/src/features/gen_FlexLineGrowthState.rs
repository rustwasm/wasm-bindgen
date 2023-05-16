#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `FlexLineGrowthState` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `FlexLineGrowthState`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlexLineGrowthState {
    Unchanged = "unchanged",
    Shrinking = "shrinking",
    Growing = "growing",
}
