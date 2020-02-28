use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `RtcStatsIceCandidatePairState` enum.\n\n*This API requires the following crate features to be activated: `RtcStatsIceCandidatePairState`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum RtcStatsIceCandidatePairState {
    Frozen = "frozen",
    Waiting = "waiting",
    Inprogress = "inprogress",
    Failed = "failed",
    Succeeded = "succeeded",
    Cancelled = "cancelled",
}
