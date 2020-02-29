use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `MediaEncodingType` enum.
///
///*This API requires the following crate features to be activated: `MediaEncodingType`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum MediaEncodingType {
    Record = "record",
    Transmission = "transmission",
}
