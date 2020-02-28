use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = EventTarget , extends = :: js_sys :: Object , js_name = ImageCapture , typescript_name = ImageCapture ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ImageCapture` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageCapture)\n\n*This API requires the following crate features to be activated: `ImageCapture`*"]
    pub type ImageCapture;
    # [ wasm_bindgen ( structural , method , getter , js_name = videoStreamTrack ) ]
    #[cfg(feature = "VideoStreamTrack")]
    #[doc = "Getter for the `videoStreamTrack` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageCapture/videoStreamTrack)\n\n*This API requires the following crate features to be activated: `ImageCapture`, `VideoStreamTrack`*"]
    pub fn video_stream_track(this: &ImageCapture) -> VideoStreamTrack;
    # [ wasm_bindgen ( structural , method , getter , js_name = onphoto ) ]
    #[doc = "Getter for the `onphoto` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageCapture/onphoto)\n\n*This API requires the following crate features to be activated: `ImageCapture`*"]
    pub fn onphoto(this: &ImageCapture) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onphoto ) ]
    #[doc = "Setter for the `onphoto` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageCapture/onphoto)\n\n*This API requires the following crate features to be activated: `ImageCapture`*"]
    pub fn set_onphoto(this: &ImageCapture, value: Option<::js_sys::Function>);
    # [ wasm_bindgen ( structural , method , getter , js_name = onerror ) ]
    #[doc = "Getter for the `onerror` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageCapture/onerror)\n\n*This API requires the following crate features to be activated: `ImageCapture`*"]
    pub fn onerror(this: &ImageCapture) -> Option<::js_sys::Function>;
    # [ wasm_bindgen ( structural , method , setter , js_name = onerror ) ]
    #[doc = "Setter for the `onerror` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageCapture/onerror)\n\n*This API requires the following crate features to be activated: `ImageCapture`*"]
    pub fn set_onerror(this: &ImageCapture, value: Option<::js_sys::Function>);
    #[cfg(feature = "VideoStreamTrack")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new ImageCapture(..)` constructor, creating a new instance of `ImageCapture`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageCapture/ImageCapture)\n\n*This API requires the following crate features to be activated: `ImageCapture`, `VideoStreamTrack`*"]
    pub fn new(this: &ImageCapture, track: &VideoStreamTrack) -> Result<ImageCapture, JsValue>;
    # [ wasm_bindgen ( catch , method , structural , js_name = takePhoto ) ]
    #[doc = "The `takePhoto()` method.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageCapture/takePhoto)\n\n*This API requires the following crate features to be activated: `ImageCapture`*"]
    pub fn take_photo(this: &ImageCapture) -> Result<(), JsValue>;
}
