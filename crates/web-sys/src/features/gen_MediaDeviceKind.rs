#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen(skip_typescript)]
#[doc = "The `MediaDeviceKind` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `MediaDeviceKind`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaDeviceKind {
    Audioinput = "audioinput",
    Audiooutput = "audiooutput",
    Videoinput = "videoinput",
}
