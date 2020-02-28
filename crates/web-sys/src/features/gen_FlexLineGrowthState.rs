use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `FlexLineGrowthState` enum.\n\n*This API requires the following crate features to be activated: `FlexLineGrowthState`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum FlexLineGrowthState {
    Unchanged = "unchanged",
    Shrinking = "shrinking",
    Growing = "growing",
}
