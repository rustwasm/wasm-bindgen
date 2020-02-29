use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `RtcRtpTransceiverDirection` enum.
///
///*This API requires the following crate features to be activated: `RtcRtpTransceiverDirection`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum RtcRtpTransceiverDirection {
    Sendrecv = "sendrecv",
    Sendonly = "sendonly",
    Recvonly = "recvonly",
    Inactive = "inactive",
}
