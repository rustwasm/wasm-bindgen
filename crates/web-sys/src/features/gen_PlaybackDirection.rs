use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `PlaybackDirection` enum.
///
///*This API requires the following crate features to be activated: `PlaybackDirection`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum PlaybackDirection {
    Normal = "normal",
    Reverse = "reverse",
    Alternate = "alternate",
    AlternateReverse = "alternate-reverse",
}
