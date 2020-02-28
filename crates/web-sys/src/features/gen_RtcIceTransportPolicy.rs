use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `RtcIceTransportPolicy` enum.\n\n*This API requires the following crate features to be activated: `RtcIceTransportPolicy`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum RtcIceTransportPolicy {
    Relay = "relay",
    All = "all",
}
