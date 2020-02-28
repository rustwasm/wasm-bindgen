use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `FontFaceLoadStatus` enum.\n\n*This API requires the following crate features to be activated: `FontFaceLoadStatus`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum FontFaceLoadStatus {
    Unloaded = "unloaded",
    Loading = "loading",
    Loaded = "loaded",
    Error = "error",
}
