use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `PcImplSignalingState` enum.\n\n*This API requires the following crate features to be activated: `PcImplSignalingState`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PcImplSignalingState {
    SignalingInvalid = "SignalingInvalid",
    SignalingStable = "SignalingStable",
    SignalingHaveLocalOffer = "SignalingHaveLocalOffer",
    SignalingHaveRemoteOffer = "SignalingHaveRemoteOffer",
    SignalingHaveLocalPranswer = "SignalingHaveLocalPranswer",
    SignalingHaveRemotePranswer = "SignalingHaveRemotePranswer",
    SignalingClosed = "SignalingClosed",
}
