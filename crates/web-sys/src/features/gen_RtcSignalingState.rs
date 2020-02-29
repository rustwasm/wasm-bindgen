use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `RtcSignalingState` enum.
///
///*This API requires the following crate features to be activated: `RtcSignalingState`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum RtcSignalingState {
    Stable = "stable",
    HaveLocalOffer = "have-local-offer",
    HaveRemoteOffer = "have-remote-offer",
    HaveLocalPranswer = "have-local-pranswer",
    HaveRemotePranswer = "have-remote-pranswer",
    Closed = "closed",
}
