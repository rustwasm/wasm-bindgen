use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `AttestationConveyancePreference` enum.\n\n*This API requires the following crate features to be activated: `AttestationConveyancePreference`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum AttestationConveyancePreference {
    None = "none",
    Indirect = "indirect",
    Direct = "direct",
}
