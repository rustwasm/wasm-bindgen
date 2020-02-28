use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [ wasm_bindgen ( extends = Event , extends = :: js_sys :: Object , js_name = ImageCaptureErrorEvent , typescript_name = ImageCaptureErrorEvent ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ImageCaptureErrorEvent` class.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageCaptureErrorEvent)\n\n*This API requires the following crate features to be activated: `ImageCaptureErrorEvent`*"]
    pub type ImageCaptureErrorEvent;
    # [ wasm_bindgen ( structural , method , getter , js_name = imageCaptureError ) ]
    #[cfg(feature = "ImageCaptureError")]
    #[doc = "Getter for the `imageCaptureError` field of this object.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageCaptureErrorEvent/imageCaptureError)\n\n*This API requires the following crate features to be activated: `ImageCaptureError`, `ImageCaptureErrorEvent`*"]
    pub fn image_capture_error(this: &ImageCaptureErrorEvent) -> Option<ImageCaptureError>;
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new ImageCaptureErrorEvent(..)` constructor, creating a new instance of `ImageCaptureErrorEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageCaptureErrorEvent/ImageCaptureErrorEvent)\n\n*This API requires the following crate features to be activated: `ImageCaptureErrorEvent`*"]
    pub fn new(
        this: &ImageCaptureErrorEvent,
        type_: &str,
    ) -> Result<ImageCaptureErrorEvent, JsValue>;
    #[cfg(feature = "ImageCaptureErrorEventInit")]
    #[wasm_bindgen(catch, constructor)]
    #[doc = "The `new ImageCaptureErrorEvent(..)` constructor, creating a new instance of `ImageCaptureErrorEvent`.\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageCaptureErrorEvent/ImageCaptureErrorEvent)\n\n*This API requires the following crate features to be activated: `ImageCaptureErrorEvent`, `ImageCaptureErrorEventInit`*"]
    pub fn new_with_image_capture_error_init_dict(
        this: &ImageCaptureErrorEvent,
        type_: &str,
        image_capture_error_init_dict: &ImageCaptureErrorEventInit,
    ) -> Result<ImageCaptureErrorEvent, JsValue>;
}
