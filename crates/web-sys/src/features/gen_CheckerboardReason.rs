use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `CheckerboardReason` enum.
///
///*This API requires the following crate features to be activated: `CheckerboardReason`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum CheckerboardReason {
    Severe = "severe",
    Recent = "recent",
}
