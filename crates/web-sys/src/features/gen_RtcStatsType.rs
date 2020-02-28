use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `RtcStatsType` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcStatsType`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum RtcStatsType {
    InboundRtp = "inbound-rtp",
    OutboundRtp = "outbound-rtp",
    Csrc = "csrc",
    Session = "session",
    Track = "track",
    Transport = "transport",
    CandidatePair = "candidate-pair",
    LocalCandidate = "local-candidate",
    RemoteCandidate = "remote-candidate",
}
