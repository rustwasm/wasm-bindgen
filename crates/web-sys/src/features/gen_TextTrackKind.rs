use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `TextTrackKind` enum.\n\n*This API requires the following crate features to be activated: `TextTrackKind`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TextTrackKind {
    Subtitles = "subtitles",
    Captions = "captions",
    Descriptions = "descriptions",
    Chapters = "chapters",
    Metadata = "metadata",
}
