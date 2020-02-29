use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = MediaError , typescript_type = "MediaError" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `MediaError` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaError)
    ///
    ///*This API requires the following crate features to be activated: `MediaError`*
    pub type MediaError;

    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaError" , js_name = code ) ]
    ///Getter for the `code` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaError/code)
    ///
    ///*This API requires the following crate features to be activated: `MediaError`*
    pub fn code(this: &MediaError) -> u16;

    # [ wasm_bindgen ( structural , method , getter , js_class = "MediaError" , js_name = message ) ]
    ///Getter for the `message` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaError/message)
    ///
    ///*This API requires the following crate features to be activated: `MediaError`*
    pub fn message(this: &MediaError) -> String;

}

impl MediaError {
    ///The `MediaError.MEDIA_ERR_ABORTED` const.
    ///
    ///*This API requires the following crate features to be activated: `MediaError`*

    pub const MEDIA_ERR_ABORTED: u16 = 1u64 as u16;

    ///The `MediaError.MEDIA_ERR_NETWORK` const.
    ///
    ///*This API requires the following crate features to be activated: `MediaError`*

    pub const MEDIA_ERR_NETWORK: u16 = 2u64 as u16;

    ///The `MediaError.MEDIA_ERR_DECODE` const.
    ///
    ///*This API requires the following crate features to be activated: `MediaError`*

    pub const MEDIA_ERR_DECODE: u16 = 3u64 as u16;

    ///The `MediaError.MEDIA_ERR_SRC_NOT_SUPPORTED` const.
    ///
    ///*This API requires the following crate features to be activated: `MediaError`*

    pub const MEDIA_ERR_SRC_NOT_SUPPORTED: u16 = 4u64 as u16;
}
