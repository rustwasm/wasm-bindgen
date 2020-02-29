use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `CompositeOperation` enum.
///
///*This API requires the following crate features to be activated: `CompositeOperation`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum CompositeOperation {
    Replace = "replace",
    Add = "add",
    Accumulate = "accumulate",
}
