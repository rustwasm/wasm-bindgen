use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `MidiPortConnectionState` enum.
///
///*This API requires the following crate features to be activated: `MidiPortConnectionState`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum MidiPortConnectionState {
    Open = "open",
    Closed = "closed",
    Pending = "pending",
}
