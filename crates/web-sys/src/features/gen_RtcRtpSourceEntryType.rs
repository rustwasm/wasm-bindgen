#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen(no_export)]
#[doc = "The `RtcRtpSourceEntryType` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcRtpSourceEntryType`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RtcRtpSourceEntryType {
    Contributing = "contributing",
    Synchronization = "synchronization",
}
