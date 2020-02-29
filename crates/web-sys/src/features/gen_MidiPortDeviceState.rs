use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `MidiPortDeviceState` enum.
///
///*This API requires the following crate features to be activated: `MidiPortDeviceState`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum MidiPortDeviceState {
    Disconnected = "disconnected",
    Connected = "connected",
}
