use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `ScreenColorGamut` enum.\n\n*This API requires the following crate features to be activated: `ScreenColorGamut`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ScreenColorGamut {
    Srgb = "srgb",
    P3 = "p3",
    Rec2020 = "rec2020",
}
