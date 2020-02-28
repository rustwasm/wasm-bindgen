use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `RequestCredentials` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RequestCredentials`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum RequestCredentials {
    Omit = "omit",
    SameOrigin = "same-origin",
    Include = "include",
}
