use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `MidiPortDeviceState` enum.\n\n*This API requires the following crate features to be activated: `MidiPortDeviceState`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MidiPortDeviceState {
    Disconnected = "disconnected",
    Connected = "connected",
}
