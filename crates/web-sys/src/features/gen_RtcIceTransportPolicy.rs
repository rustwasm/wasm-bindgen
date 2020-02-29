use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `RtcIceTransportPolicy` enum.
///
///*This API requires the following crate features to be activated: `RtcIceTransportPolicy`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum RtcIceTransportPolicy {
    Relay = "relay",
    All = "all",
}
