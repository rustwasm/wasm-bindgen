use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `PcImplIceGatheringState` enum.\n\n*This API requires the following crate features to be activated: `PcImplIceGatheringState`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PcImplIceGatheringState {
    New = "new",
    Gathering = "gathering",
    Complete = "complete",
}
