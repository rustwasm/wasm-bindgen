use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `MediaDeviceKind` enum.
///
///*This API requires the following crate features to be activated: `MediaDeviceKind`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum MediaDeviceKind {
    Audioinput = "audioinput",
    Audiooutput = "audiooutput",
    Videoinput = "videoinput",
}
