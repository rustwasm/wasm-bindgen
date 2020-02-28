use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `MediaSourceEndOfStreamError` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `MediaSourceEndOfStreamError`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MediaSourceEndOfStreamError {
    Network = "network",
    Decode = "decode",
}
