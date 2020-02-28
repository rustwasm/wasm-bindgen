use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `RtcSignalingState` enum.\n\n*This API requires the following crate features to be activated: `RtcSignalingState`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum RtcSignalingState {
    Stable = "stable",
    HaveLocalOffer = "have-local-offer",
    HaveRemoteOffer = "have-remote-offer",
    HaveLocalPranswer = "have-local-pranswer",
    HaveRemotePranswer = "have-remote-pranswer",
    Closed = "closed",
}
