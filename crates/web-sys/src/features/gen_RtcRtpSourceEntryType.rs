use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `RtcRtpSourceEntryType` enum.\n\n*This API requires the following crate features to be activated: `RtcRtpSourceEntryType`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum RtcRtpSourceEntryType {
    Contributing = "contributing",
    Synchronization = "synchronization",
}
