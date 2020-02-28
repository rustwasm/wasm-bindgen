use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `BinaryType` enum.\n\n*This API requires the following crate features to be activated: `BinaryType`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum BinaryType {
    Blob = "blob",
    Arraybuffer = "arraybuffer",
}
