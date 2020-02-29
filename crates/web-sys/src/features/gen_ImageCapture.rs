use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = ImageCapture , typescript_name = ImageCapture ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `ImageCapture` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageCapture)
    ///
    ///*This API requires the following crate features to be activated: `ImageCapture`*
    pub type ImageCapture;

    #[cfg(feature = "VideoStreamTrack")]
    # [ wasm_bindgen ( structural , method , getter , js_class = "ImageCapture" , js_name = videoStreamTrack ) ]
    ///Getter for the `videoStreamTrack` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageCapture/videoStreamTrack)
    ///
    ///*This API requires the following crate features to be activated: `ImageCapture`, `VideoStreamTrack`*
    pub fn video_stream_track(this: &ImageCapture) -> VideoStreamTrack;

    # [ wasm_bindgen ( structural , method , getter , js_class = "ImageCapture" , js_name = onphoto ) ]
    ///Getter for the `onphoto` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageCapture/onphoto)
    ///
    ///*This API requires the following crate features to be activated: `ImageCapture`*
    pub fn onphoto(this: &ImageCapture) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "ImageCapture" , js_name = onphoto ) ]
    ///Setter for the `onphoto` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageCapture/onphoto)
    ///
    ///*This API requires the following crate features to be activated: `ImageCapture`*
    pub fn set_onphoto(this: &ImageCapture, value: Option<&::js_sys::Function>);

    # [ wasm_bindgen ( structural , method , getter , js_class = "ImageCapture" , js_name = onerror ) ]
    ///Getter for the `onerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageCapture/onerror)
    ///
    ///*This API requires the following crate features to be activated: `ImageCapture`*
    pub fn onerror(this: &ImageCapture) -> Option<::js_sys::Function>;

    # [ wasm_bindgen ( structural , method , setter , js_class = "ImageCapture" , js_name = onerror ) ]
    ///Setter for the `onerror` field of this object.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageCapture/onerror)
    ///
    ///*This API requires the following crate features to be activated: `ImageCapture`*
    pub fn set_onerror(this: &ImageCapture, value: Option<&::js_sys::Function>);

    #[cfg(feature = "VideoStreamTrack")]
    #[wasm_bindgen(catch, constructor, js_class = "ImageCapture")]
    ///The `new ImageCapture(..)` constructor, creating a new instance of `ImageCapture`.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageCapture/ImageCapture)
    ///
    ///*This API requires the following crate features to be activated: `ImageCapture`, `VideoStreamTrack`*
    pub fn new(track: &VideoStreamTrack) -> Result<ImageCapture, JsValue>;

    # [ wasm_bindgen ( catch , method , structural , js_class = "ImageCapture" , js_name = takePhoto ) ]
    ///The `takePhoto()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageCapture/takePhoto)
    ///
    ///*This API requires the following crate features to be activated: `ImageCapture`*
    pub fn take_photo(this: &ImageCapture) -> Result<(), JsValue>;

}
