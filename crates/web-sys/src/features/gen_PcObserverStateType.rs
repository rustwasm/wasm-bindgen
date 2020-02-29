use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `PcObserverStateType` enum.
///
///*This API requires the following crate features to be activated: `PcObserverStateType`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum PcObserverStateType {
    None = "None",
    IceConnectionState = "IceConnectionState",
    IceGatheringState = "IceGatheringState",
    SignalingState = "SignalingState",
}
