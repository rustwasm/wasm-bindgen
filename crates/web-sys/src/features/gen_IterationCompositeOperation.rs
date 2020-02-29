use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `IterationCompositeOperation` enum.
///
///*This API requires the following crate features to be activated: `IterationCompositeOperation`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum IterationCompositeOperation {
    Replace = "replace",
    Accumulate = "accumulate",
}
