use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = MediaKeyError , typescript_type = "MediaKeyError" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `MediaKeyError` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyError)
    ///
    ///*This API requires the following crate features to be activated: `MediaKeyError`*
    pub type MediaKeyError;

    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaKeyError" , js_name = systemCode ) ]
    ///Getter for the `systemCode` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyError/systemCode)
    ///
    ///*This API requires the following crate features to be activated: `MediaKeyError`*
    pub fn system_code(this: &MediaKeyError) -> u32;

}
