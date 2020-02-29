use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `RtcStatsIceCandidatePairState` enum.
///
///*This API requires the following crate features to be activated: `RtcStatsIceCandidatePairState`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum RtcStatsIceCandidatePairState {
    Frozen = "frozen",
    Waiting = "waiting",
    Inprogress = "inprogress",
    Failed = "failed",
    Succeeded = "succeeded",
    Cancelled = "cancelled",
}
