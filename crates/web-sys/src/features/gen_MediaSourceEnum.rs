use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `MediaSourceEnum` enum.\n\n*This API requires the following crate features to be activated: `MediaSourceEnum`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MediaSourceEnum {
    Camera = "camera",
    Screen = "screen",
    Application = "application",
    Window = "window",
    Browser = "browser",
    Microphone = "microphone",
    AudioCapture = "audioCapture",
    Other = "other",
}
