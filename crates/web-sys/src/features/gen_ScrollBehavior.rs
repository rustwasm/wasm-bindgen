use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `ScrollBehavior` enum.
///
///*This API requires the following crate features to be activated: `ScrollBehavior`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum ScrollBehavior {
    Auto = "auto",
    Instant = "instant",
    Smooth = "smooth",
}
