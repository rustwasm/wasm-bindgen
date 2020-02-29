use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `VisibilityState` enum.
///
///*This API requires the following crate features to be activated: `VisibilityState`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum VisibilityState {
    Hidden = "hidden",
    Visible = "visible",
}
