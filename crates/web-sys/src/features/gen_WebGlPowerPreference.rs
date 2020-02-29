use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `WebGlPowerPreference` enum.
///
///*This API requires the following crate features to be activated: `WebGlPowerPreference`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum WebGlPowerPreference {
    Default = "default",
    LowPower = "low-power",
    HighPerformance = "high-performance",
}
