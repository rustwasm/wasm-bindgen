use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `BiquadFilterType` enum.
///
///*This API requires the following crate features to be activated: `BiquadFilterType`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum BiquadFilterType {
    Lowpass = "lowpass",
    Highpass = "highpass",
    Bandpass = "bandpass",
    Lowshelf = "lowshelf",
    Highshelf = "highshelf",
    Peaking = "peaking",
    Notch = "notch",
    Allpass = "allpass",
}
