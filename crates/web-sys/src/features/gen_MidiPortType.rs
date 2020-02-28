use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `MidiPortType` enum.\n\n*This API requires the following crate features to be activated: `MidiPortType`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MidiPortType {
    Input = "input",
    Output = "output",
}
