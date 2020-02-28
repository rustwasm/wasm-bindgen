use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `MediaDeviceKind` enum.\n\n*This API requires the following crate features to be activated: `MediaDeviceKind`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MediaDeviceKind {
    Audioinput = "audioinput",
    Audiooutput = "audiooutput",
    Videoinput = "videoinput",
}
