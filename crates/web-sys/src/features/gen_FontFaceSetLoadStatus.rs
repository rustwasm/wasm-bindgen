use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `FontFaceSetLoadStatus` enum.
///
///*This API requires the following crate features to be activated: `FontFaceSetLoadStatus`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum FontFaceSetLoadStatus {
    Loading = "loading",
    Loaded = "loaded",
}
