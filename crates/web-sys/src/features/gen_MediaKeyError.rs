use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = MediaKeyError , typescript_name = MediaKeyError ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaKeyError` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyError)\n\n*This API requires the following crate features to be activated: `MediaKeyError`*"]
    pub type MediaKeyError;
    # [ wasm_bindgen ( structural , method , getter , js_name = systemCode ) ]
    #[doc = "Getter for the `systemCode` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyError/systemCode)\n\n*This API requires the following crate features to be activated: `MediaKeyError`*"]
    pub fn system_code(this: &MediaKeyError) -> u32;
}
