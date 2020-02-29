use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `RtcIceGatheringState` enum.
///
///*This API requires the following crate features to be activated: `RtcIceGatheringState`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum RtcIceGatheringState {
    New = "new",
    Gathering = "gathering",
    Complete = "complete",
}
