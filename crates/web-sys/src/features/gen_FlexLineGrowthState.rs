use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `FlexLineGrowthState` enum.
///
///*This API requires the following crate features to be activated: `FlexLineGrowthState`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum FlexLineGrowthState {
    Unchanged = "unchanged",
    Shrinking = "shrinking",
    Growing = "growing",
}
