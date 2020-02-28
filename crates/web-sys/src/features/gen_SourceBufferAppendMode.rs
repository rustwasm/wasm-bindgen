use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `SourceBufferAppendMode` enum.\n\n*This API requires the following crate features to be activated: `SourceBufferAppendMode`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum SourceBufferAppendMode {
    Segments = "segments",
    Sequence = "sequence",
}
