use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `TextTrackKind` enum.
///
///*This API requires the following crate features to be activated: `TextTrackKind`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum TextTrackKind {
    Subtitles = "subtitles",
    Captions = "captions",
    Descriptions = "descriptions",
    Chapters = "chapters",
    Metadata = "metadata",
}
