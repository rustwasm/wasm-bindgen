use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `PlaybackDirection` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `PlaybackDirection`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PlaybackDirection {
    Normal = "normal",
    Reverse = "reverse",
    Alternate = "alternate",
    AlternateReverse = "alternate-reverse",
}
