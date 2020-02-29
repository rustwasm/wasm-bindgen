use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `RtcStatsIceCandidateType` enum.
///
///*This API requires the following crate features to be activated: `RtcStatsIceCandidateType`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum RtcStatsIceCandidateType {
    Host = "host",
    Serverreflexive = "serverreflexive",
    Peerreflexive = "peerreflexive",
    Relayed = "relayed",
}
