use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `RtcStatsIceCandidateType` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcStatsIceCandidateType`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum RtcStatsIceCandidateType {
    Host = "host",
    Serverreflexive = "serverreflexive",
    Peerreflexive = "peerreflexive",
    Relayed = "relayed",
}
