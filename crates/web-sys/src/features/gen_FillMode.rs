use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `FillMode` enum.\n\n*This API requires the following crate features to be activated: `FillMode`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum FillMode {
    None = "none",
    Forwards = "forwards",
    Backwards = "backwards",
    Both = "both",
    Auto = "auto",
}
