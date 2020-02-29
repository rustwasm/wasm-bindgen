use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `BinaryType` enum.
///
///*This API requires the following crate features to be activated: `BinaryType`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum BinaryType {
    Blob = "blob",
    Arraybuffer = "arraybuffer",
}
