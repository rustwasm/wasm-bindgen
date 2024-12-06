#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen(no_export)]
#[doc = "The `MediaDecodingType` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `MediaDecodingType`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaDecodingType {
    File = "file",
    MediaSource = "media-source",
}
