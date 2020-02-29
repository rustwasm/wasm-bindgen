use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `RtcRtpSourceEntryType` enum.
///
///*This API requires the following crate features to be activated: `RtcRtpSourceEntryType`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum RtcRtpSourceEntryType {
    Contributing = "contributing",
    Synchronization = "synchronization",
}
