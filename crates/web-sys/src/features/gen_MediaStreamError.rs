use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = MediaStreamError , typescript_name = MediaStreamError ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaStreamError` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamError)\n\n*This API requires the following crate features to be activated: `MediaStreamError`*"]
    pub type MediaStreamError;
    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaStreamError" , js_name = name ) ]
    #[doc = "Getter for the `name` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamError/name)\n\n*This API requires the following crate features to be activated: `MediaStreamError`*"]
    pub fn name(this: &MediaStreamError) -> String;
    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaStreamError" , js_name = message ) ]
    #[doc = "Getter for the `message` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamError/message)\n\n*This API requires the following crate features to be activated: `MediaStreamError`*"]
    pub fn message(this: &MediaStreamError) -> Option<String>;
    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaStreamError" , js_name = constraint ) ]
    #[doc = "Getter for the `constraint` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamError/constraint)\n\n*This API requires the following crate features to be activated: `MediaStreamError`*"]
    pub fn constraint(this: &MediaStreamError) -> Option<String>;
}
