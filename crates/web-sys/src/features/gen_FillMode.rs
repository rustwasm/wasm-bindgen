use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `FillMode` enum.
///
///*This API requires the following crate features to be activated: `FillMode`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum FillMode {
    None = "none",
    Forwards = "forwards",
    Backwards = "backwards",
    Both = "both",
    Auto = "auto",
}
