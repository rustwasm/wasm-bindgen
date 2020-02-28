use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = MediaError , typescript_name = MediaError ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaError` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaError)\n\n*This API requires the following crate features to be activated: `MediaError`*"]
    pub type MediaError;
    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaError" , js_name = code ) ]
    #[doc = "Getter for the `code` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaError/code)\n\n*This API requires the following crate features to be activated: `MediaError`*"]
    pub fn code(this: &MediaError) -> u16;
    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaError" , js_name = message ) ]
    #[doc = "Getter for the `message` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaError/message)\n\n*This API requires the following crate features to be activated: `MediaError`*"]
    pub fn message(this: &MediaError) -> String;
}
impl MediaError {
    pub const MEDIA_ERR_ABORTED: u16 = 1u64 as u16;
    pub const MEDIA_ERR_NETWORK: u16 = 2u64 as u16;
    pub const MEDIA_ERR_DECODE: u16 = 3u64 as u16;
    pub const MEDIA_ERR_SRC_NOT_SUPPORTED: u16 = 4u64 as u16;
}
