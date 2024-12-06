#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen(no_export)]
#[doc = "The `RtcDataChannelType` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcDataChannelType`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RtcDataChannelType {
    Arraybuffer = "arraybuffer",
    Blob = "blob",
}
