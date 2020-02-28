use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `RtcRtpTransceiverDirection` enum.\n\n*This API requires the following crate features to be activated: `RtcRtpTransceiverDirection`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum RtcRtpTransceiverDirection {
    Sendrecv = "sendrecv",
    Sendonly = "sendonly",
    Recvonly = "recvonly",
    Inactive = "inactive",
}
