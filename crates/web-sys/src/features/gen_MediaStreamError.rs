use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = MediaStreamError , typescript_name = MediaStreamError ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `MediaStreamError` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamError)
    ///
    ///*This API requires the following crate features to be activated: `MediaStreamError`*
    pub type MediaStreamError;

    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaStreamError" , js_name = name ) ]
    ///Getter for the `name` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamError/name)
    ///
    ///*This API requires the following crate features to be activated: `MediaStreamError`*
    pub fn name(this: &MediaStreamError) -> String;

    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaStreamError" , js_name = message ) ]
    ///Getter for the `message` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamError/message)
    ///
    ///*This API requires the following crate features to be activated: `MediaStreamError`*
    pub fn message(this: &MediaStreamError) -> Option<String>;

    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaStreamError" , js_name = constraint ) ]
    ///Getter for the `constraint` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamError/constraint)
    ///
    ///*This API requires the following crate features to be activated: `MediaStreamError`*
    pub fn constraint(this: &MediaStreamError) -> Option<String>;

}
