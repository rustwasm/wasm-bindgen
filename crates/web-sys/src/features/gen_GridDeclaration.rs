use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `GridDeclaration` enum.
///
///*This API requires the following crate features to be activated: `GridDeclaration`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum GridDeclaration {
    Explicit = "explicit",
    Implicit = "implicit",
}
