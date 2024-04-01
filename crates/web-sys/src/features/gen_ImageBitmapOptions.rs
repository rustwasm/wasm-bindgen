#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ImageBitmapOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ImageBitmapOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`*"]
    pub type ImageBitmapOptions;
    #[cfg(feature = "ColorSpaceConversion")]
    #[wasm_bindgen(method, setter = "colorSpaceConversion")]
    fn color_space_conversion_shim(this: &ImageBitmapOptions, val: ColorSpaceConversion);
    #[cfg(feature = "ImageOrientation")]
    #[wasm_bindgen(method, setter = "imageOrientation")]
    fn image_orientation_shim(this: &ImageBitmapOptions, val: ImageOrientation);
    #[cfg(feature = "PremultiplyAlpha")]
    #[wasm_bindgen(method, setter = "premultiplyAlpha")]
    fn premultiply_alpha_shim(this: &ImageBitmapOptions, val: PremultiplyAlpha);
    #[wasm_bindgen(method, setter = "resizeHeight")]
    fn resize_height_shim(this: &ImageBitmapOptions, val: u32);
    #[cfg(feature = "ResizeQuality")]
    #[wasm_bindgen(method, setter = "resizeQuality")]
    fn resize_quality_shim(this: &ImageBitmapOptions, val: ResizeQuality);
    #[wasm_bindgen(method, setter = "resizeWidth")]
    fn resize_width_shim(this: &ImageBitmapOptions, val: u32);
}
impl ImageBitmapOptions {
    #[doc = "Construct a new `ImageBitmapOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "ColorSpaceConversion")]
    #[doc = "Change the `colorSpaceConversion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ColorSpaceConversion`, `ImageBitmapOptions`*"]
    pub fn color_space_conversion(&mut self, val: ColorSpaceConversion) -> &mut Self {
        self.color_space_conversion_shim(val);
        self
    }
    #[cfg(feature = "ImageOrientation")]
    #[doc = "Change the `imageOrientation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`, `ImageOrientation`*"]
    pub fn image_orientation(&mut self, val: ImageOrientation) -> &mut Self {
        self.image_orientation_shim(val);
        self
    }
    #[cfg(feature = "PremultiplyAlpha")]
    #[doc = "Change the `premultiplyAlpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`, `PremultiplyAlpha`*"]
    pub fn premultiply_alpha(&mut self, val: PremultiplyAlpha) -> &mut Self {
        self.premultiply_alpha_shim(val);
        self
    }
    #[doc = "Change the `resizeHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`*"]
    pub fn resize_height(&mut self, val: u32) -> &mut Self {
        self.resize_height_shim(val);
        self
    }
    #[cfg(feature = "ResizeQuality")]
    #[doc = "Change the `resizeQuality` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`, `ResizeQuality`*"]
    pub fn resize_quality(&mut self, val: ResizeQuality) -> &mut Self {
        self.resize_quality_shim(val);
        self
    }
    #[doc = "Change the `resizeWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`*"]
    pub fn resize_width(&mut self, val: u32) -> &mut Self {
        self.resize_width_shim(val);
        self
    }
}
impl Default for ImageBitmapOptions {
    fn default() -> Self {
        Self::new()
    }
}
