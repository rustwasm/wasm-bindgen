#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `GridDeclaration` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `GridDeclaration`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridDeclaration {
    Explicit = "explicit",
    Implicit = "implicit",
}
