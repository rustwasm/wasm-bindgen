use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `MidiPortType` enum.
///
///*This API requires the following crate features to be activated: `MidiPortType`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum MidiPortType {
    Input = "input",
    Output = "output",
}
