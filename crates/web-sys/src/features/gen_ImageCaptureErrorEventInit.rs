#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ImageCaptureErrorEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ImageCaptureErrorEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageCaptureErrorEventInit`*"]
    pub type ImageCaptureErrorEventInit;
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &ImageCaptureErrorEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &ImageCaptureErrorEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &ImageCaptureErrorEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &ImageCaptureErrorEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &ImageCaptureErrorEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &ImageCaptureErrorEventInit, val: bool);
    #[cfg(feature = "ImageCaptureError")]
    #[wasm_bindgen(method, getter = "imageCaptureError")]
    fn image_capture_error_shim(this: &ImageCaptureErrorEventInit) -> Option<&ImageCaptureError>;
    #[cfg(feature = "ImageCaptureError")]
    #[wasm_bindgen(method, setter = "imageCaptureError")]
    fn set_image_capture_error_shim(
        this: &ImageCaptureErrorEventInit,
        val: Option<&ImageCaptureError>,
    );
}
#[doc = "The trait to access properties on the `ImageCaptureErrorEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ImageCaptureErrorEventInit`*"]
pub trait ImageCaptureErrorEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageCaptureErrorEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageCaptureErrorEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageCaptureErrorEventInit`*"]
    fn composed(&self) -> bool;
    #[cfg(feature = "ImageCaptureError")]
    #[doc = "Get the `imageCaptureError` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageCaptureError`, `ImageCaptureErrorEventInit`*"]
    fn image_capture_error(&self) -> Option<&ImageCaptureError>;
}
impl ImageCaptureErrorEventInitGetters for ImageCaptureErrorEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    #[cfg(feature = "ImageCaptureError")]
    fn image_capture_error(&self) -> Option<&ImageCaptureError> {
        self.image_capture_error_shim()
    }
}
impl ImageCaptureErrorEventInit {
    #[doc = "Construct a new `ImageCaptureErrorEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageCaptureErrorEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageCaptureErrorEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageCaptureErrorEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageCaptureErrorEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[cfg(feature = "ImageCaptureError")]
    #[doc = "Change the `imageCaptureError` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageCaptureError`, `ImageCaptureErrorEventInit`*"]
    pub fn image_capture_error(&mut self, val: Option<&ImageCaptureError>) -> &mut Self {
        self.set_image_capture_error_shim(val);
        self
    }
}
impl Default for ImageCaptureErrorEventInit {
    fn default() -> Self {
        Self::new()
    }
}
