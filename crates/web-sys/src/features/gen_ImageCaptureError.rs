use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( is_type_of = | _ | false , extends = :: js_sys :: Object , js_name = ImageCaptureError , typescript_name = ImageCaptureError ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `ImageCaptureError` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageCaptureError)
    ///
    ///*This API requires the following crate features to be activated: `ImageCaptureError`*
    pub type ImageCaptureError;

    # [ wasm_bindgen ( structural , method , getter , js_class = "ImageCaptureError" , js_name = code ) ]
    ///Getter for the `code` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageCaptureError/code)
    ///
    ///*This API requires the following crate features to be activated: `ImageCaptureError`*
    pub fn code(this: &ImageCaptureError) -> u16;

    # [ wasm_bindgen ( structural , method , getter , js_class = "ImageCaptureError" , js_name = message ) ]
    ///Getter for the `message` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageCaptureError/message)
    ///
    ///*This API requires the following crate features to be activated: `ImageCaptureError`*
    pub fn message(this: &ImageCaptureError) -> String;

}

impl ImageCaptureError {
    ///The `ImageCaptureError.FRAME_GRAB_ERROR` const.
    ///
    ///*This API requires the following crate features to be activated: `ImageCaptureError`*

    pub const FRAME_GRAB_ERROR: u16 = 1u64 as u16;

    ///The `ImageCaptureError.SETTINGS_ERROR` const.
    ///
    ///*This API requires the following crate features to be activated: `ImageCaptureError`*

    pub const SETTINGS_ERROR: u16 = 2u64 as u16;

    ///The `ImageCaptureError.PHOTO_ERROR` const.
    ///
    ///*This API requires the following crate features to be activated: `ImageCaptureError`*

    pub const PHOTO_ERROR: u16 = 3u64 as u16;

    ///The `ImageCaptureError.ERROR_UNKNOWN` const.
    ///
    ///*This API requires the following crate features to be activated: `ImageCaptureError`*

    pub const ERROR_UNKNOWN: u16 = 4u64 as u16;
}
