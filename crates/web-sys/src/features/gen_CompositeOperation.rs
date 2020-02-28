use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `CompositeOperation` enum.\n\n*This API requires the following crate features to be activated: `CompositeOperation`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CompositeOperation {
    Replace = "replace",
    Add = "add",
    Accumulate = "accumulate",
}
