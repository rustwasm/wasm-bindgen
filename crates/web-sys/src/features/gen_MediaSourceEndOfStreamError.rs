use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `MediaSourceEndOfStreamError` enum.
///
///*This API requires the following crate features to be activated: `MediaSourceEndOfStreamError`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum MediaSourceEndOfStreamError {
    Network = "network",
    Decode = "decode",
}
