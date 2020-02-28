use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `FrameType` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `FrameType`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum FrameType {
    Auxiliary = "auxiliary",
    TopLevel = "top-level",
    Nested = "nested",
    None = "none",
}
