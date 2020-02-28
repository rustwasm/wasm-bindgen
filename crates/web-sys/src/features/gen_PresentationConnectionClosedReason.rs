use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `PresentationConnectionClosedReason` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `PresentationConnectionClosedReason`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PresentationConnectionClosedReason {
    Error = "error",
    Closed = "closed",
    Wentaway = "wentaway",
}
