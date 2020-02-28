use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = MediaStreamTrack , extends = EventTarget , extends = :: js_sys :: Object , js_name = VideoStreamTrack , typescript_name = VideoStreamTrack ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VideoStreamTrack` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoStreamTrack)\n\n*This API requires the following crate features to be activated: `VideoStreamTrack`*"]
    pub type VideoStreamTrack;
}
