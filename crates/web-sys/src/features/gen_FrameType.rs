use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `FrameType` enum.
///
///*This API requires the following crate features to be activated: `FrameType`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum FrameType {
    Auxiliary = "auxiliary",
    TopLevel = "top-level",
    Nested = "nested",
    None = "none",
}
