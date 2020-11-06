#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ImageCapture , typescript_type = "ImageCapture")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ImageCapture` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageCapture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageCapture`*"]
    pub type ImageCapture;
    #[cfg(feature = "MediaStreamTrack")]
    # [wasm_bindgen (structural , method , getter , js_class = "ImageCapture" , js_name = track)]
    #[doc = "Getter for the `track` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageCapture/track)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageCapture`, `MediaStreamTrack`*"]
    pub fn track(this: &ImageCapture) -> MediaStreamTrack;
    #[cfg(feature = "MediaStreamTrack")]
    #[wasm_bindgen(catch, constructor, js_class = "ImageCapture")]
    #[doc = "The `new ImageCapture(..)` constructor, creating a new instance of `ImageCapture`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageCapture/ImageCapture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageCapture`, `MediaStreamTrack`*"]
    pub fn new(video_track: &MediaStreamTrack) -> Result<ImageCapture, JsValue>;
    # [wasm_bindgen (method , structural , js_class = "ImageCapture" , js_name = getPhotoCapabilities)]
    #[doc = "The `getPhotoCapabilities()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageCapture/getPhotoCapabilities)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageCapture`*"]
    pub fn get_photo_capabilities(this: &ImageCapture) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "ImageCapture" , js_name = getPhotoSettings)]
    #[doc = "The `getPhotoSettings()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageCapture/getPhotoSettings)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageCapture`*"]
    pub fn get_photo_settings(this: &ImageCapture) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "ImageCapture" , js_name = grabFrame)]
    #[doc = "The `grabFrame()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageCapture/grabFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageCapture`*"]
    pub fn grab_frame(this: &ImageCapture) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "ImageCapture" , js_name = takePhoto)]
    #[doc = "The `takePhoto()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageCapture/takePhoto)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageCapture`*"]
    pub fn take_photo(this: &ImageCapture) -> ::js_sys::Promise;
}
