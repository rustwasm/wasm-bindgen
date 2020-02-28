use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `PcObserverStateType` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `PcObserverStateType`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PcObserverStateType {
    None = "None",
    IceConnectionState = "IceConnectionState",
    IceGatheringState = "IceGatheringState",
    SignalingState = "SignalingState",
}
