use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `RtcIceGatheringState` enum.\n\n*This API requires the following crate features to be activated: `RtcIceGatheringState`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum RtcIceGatheringState {
    New = "new",
    Gathering = "gathering",
    Complete = "complete",
}
