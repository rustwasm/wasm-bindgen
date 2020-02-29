use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `OrientationLockType` enum.
///
///*This API requires the following crate features to be activated: `OrientationLockType`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum OrientationLockType {
    Any = "any",
    Natural = "natural",
    Landscape = "landscape",
    Portrait = "portrait",
    PortraitPrimary = "portrait-primary",
    PortraitSecondary = "portrait-secondary",
    LandscapePrimary = "landscape-primary",
    LandscapeSecondary = "landscape-secondary",
}
