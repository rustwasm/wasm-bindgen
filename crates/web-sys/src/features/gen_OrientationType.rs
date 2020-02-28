use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `OrientationType` enum.\n\n*This API requires the following crate features to be activated: `OrientationType`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum OrientationType {
    PortraitPrimary = "portrait-primary",
    PortraitSecondary = "portrait-secondary",
    LandscapePrimary = "landscape-primary",
    LandscapeSecondary = "landscape-secondary",
}
