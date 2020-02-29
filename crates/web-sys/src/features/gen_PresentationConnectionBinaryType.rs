use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `PresentationConnectionBinaryType` enum.
///
///*This API requires the following crate features to be activated: `PresentationConnectionBinaryType`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum PresentationConnectionBinaryType {
    Blob = "blob",
    Arraybuffer = "arraybuffer",
}
