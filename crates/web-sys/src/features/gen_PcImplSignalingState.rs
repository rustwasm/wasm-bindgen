use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `PcImplSignalingState` enum.
///
///*This API requires the following crate features to be activated: `PcImplSignalingState`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum PcImplSignalingState {
    SignalingInvalid = "SignalingInvalid",
    SignalingStable = "SignalingStable",
    SignalingHaveLocalOffer = "SignalingHaveLocalOffer",
    SignalingHaveRemoteOffer = "SignalingHaveRemoteOffer",
    SignalingHaveLocalPranswer = "SignalingHaveLocalPranswer",
    SignalingHaveRemotePranswer = "SignalingHaveRemotePranswer",
    SignalingClosed = "SignalingClosed",
}
