use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `SourceBufferAppendMode` enum.
///
///*This API requires the following crate features to be activated: `SourceBufferAppendMode`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum SourceBufferAppendMode {
    Segments = "segments",
    Sequence = "sequence",
}
