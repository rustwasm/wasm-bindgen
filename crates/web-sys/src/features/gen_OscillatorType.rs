use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `OscillatorType` enum.\n\n*This API requires the following crate features to be activated: `OscillatorType`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum OscillatorType {
    Sine = "sine",
    Square = "square",
    Sawtooth = "sawtooth",
    Triangle = "triangle",
    Custom = "custom",
}
