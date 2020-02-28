use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `BiquadFilterType` enum.\n\n*This API requires the following crate features to be activated: `BiquadFilterType`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
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
