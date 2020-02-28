use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `WebGlPowerPreference` enum.\n\n*This API requires the following crate features to be activated: `WebGlPowerPreference`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum WebGlPowerPreference {
    Default = "default",
    LowPower = "low-power",
    HighPerformance = "high-performance",
}
