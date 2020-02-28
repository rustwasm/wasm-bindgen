use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `NavigationType` enum.\n\n*This API requires the following crate features to be activated: `NavigationType`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum NavigationType {
    Navigate = "navigate",
    Reload = "reload",
    BackForward = "back_forward",
    Prerender = "prerender",
}
