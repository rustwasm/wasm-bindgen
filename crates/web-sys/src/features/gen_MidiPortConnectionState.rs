use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `MidiPortConnectionState` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `MidiPortConnectionState`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MidiPortConnectionState {
    Open = "open",
    Closed = "closed",
    Pending = "pending",
}
