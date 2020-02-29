use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `AttestationConveyancePreference` enum.
///
///*This API requires the following crate features to be activated: `AttestationConveyancePreference`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum AttestationConveyancePreference {
    None = "none",
    Indirect = "indirect",
    Direct = "direct",
}
