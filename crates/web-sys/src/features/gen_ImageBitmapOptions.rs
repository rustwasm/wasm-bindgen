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
    #[wasm_bindgen(method, getter = "colorSpaceConversion")]
    fn color_space_conversion_shim(this: &ImageBitmapOptions) -> ColorSpaceConversion;
    #[cfg(feature = "ColorSpaceConversion")]
    #[wasm_bindgen(method, setter = "colorSpaceConversion")]
    fn set_color_space_conversion_shim(this: &ImageBitmapOptions, val: ColorSpaceConversion);
    #[cfg(feature = "ImageOrientation")]
    #[wasm_bindgen(method, getter = "imageOrientation")]
    fn image_orientation_shim(this: &ImageBitmapOptions) -> ImageOrientation;
    #[cfg(feature = "ImageOrientation")]
    #[wasm_bindgen(method, setter = "imageOrientation")]
    fn set_image_orientation_shim(this: &ImageBitmapOptions, val: ImageOrientation);
    #[cfg(feature = "PremultiplyAlpha")]
    #[wasm_bindgen(method, getter = "premultiplyAlpha")]
    fn premultiply_alpha_shim(this: &ImageBitmapOptions) -> PremultiplyAlpha;
    #[cfg(feature = "PremultiplyAlpha")]
    #[wasm_bindgen(method, setter = "premultiplyAlpha")]
    fn set_premultiply_alpha_shim(this: &ImageBitmapOptions, val: PremultiplyAlpha);
    #[wasm_bindgen(method, getter = "resizeHeight")]
    fn resize_height_shim(this: &ImageBitmapOptions) -> u32;
    #[wasm_bindgen(method, setter = "resizeHeight")]
    fn set_resize_height_shim(this: &ImageBitmapOptions, val: u32);
    #[cfg(feature = "ResizeQuality")]
    #[wasm_bindgen(method, getter = "resizeQuality")]
    fn resize_quality_shim(this: &ImageBitmapOptions) -> ResizeQuality;
    #[cfg(feature = "ResizeQuality")]
    #[wasm_bindgen(method, setter = "resizeQuality")]
    fn set_resize_quality_shim(this: &ImageBitmapOptions, val: ResizeQuality);
    #[wasm_bindgen(method, getter = "resizeWidth")]
    fn resize_width_shim(this: &ImageBitmapOptions) -> u32;
    #[wasm_bindgen(method, setter = "resizeWidth")]
    fn set_resize_width_shim(this: &ImageBitmapOptions, val: u32);
}
#[doc = "The trait to access properties on the `ImageBitmapOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`*"]
pub trait ImageBitmapOptionsGetters {
    #[cfg(feature = "ColorSpaceConversion")]
    #[doc = "Get the `colorSpaceConversion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ColorSpaceConversion`, `ImageBitmapOptions`*"]
    fn color_space_conversion(&self) -> ColorSpaceConversion;
    #[cfg(feature = "ImageOrientation")]
    #[doc = "Get the `imageOrientation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`, `ImageOrientation`*"]
    fn image_orientation(&self) -> ImageOrientation;
    #[cfg(feature = "PremultiplyAlpha")]
    #[doc = "Get the `premultiplyAlpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`, `PremultiplyAlpha`*"]
    fn premultiply_alpha(&self) -> PremultiplyAlpha;
    #[doc = "Get the `resizeHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`*"]
    fn resize_height(&self) -> u32;
    #[cfg(feature = "ResizeQuality")]
    #[doc = "Get the `resizeQuality` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`, `ResizeQuality`*"]
    fn resize_quality(&self) -> ResizeQuality;
    #[doc = "Get the `resizeWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`*"]
    fn resize_width(&self) -> u32;
}
impl ImageBitmapOptionsGetters for ImageBitmapOptions {
    #[cfg(feature = "ColorSpaceConversion")]
    fn color_space_conversion(&self) -> ColorSpaceConversion {
        self.color_space_conversion_shim()
    }
    #[cfg(feature = "ImageOrientation")]
    fn image_orientation(&self) -> ImageOrientation {
        self.image_orientation_shim()
    }
    #[cfg(feature = "PremultiplyAlpha")]
    fn premultiply_alpha(&self) -> PremultiplyAlpha {
        self.premultiply_alpha_shim()
    }
    fn resize_height(&self) -> u32 {
        self.resize_height_shim()
    }
    #[cfg(feature = "ResizeQuality")]
    fn resize_quality(&self) -> ResizeQuality {
        self.resize_quality_shim()
    }
    fn resize_width(&self) -> u32 {
        self.resize_width_shim()
    }
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
        self.set_color_space_conversion_shim(val);
        self
    }
    #[cfg(feature = "ImageOrientation")]
    #[doc = "Change the `imageOrientation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`, `ImageOrientation`*"]
    pub fn image_orientation(&mut self, val: ImageOrientation) -> &mut Self {
        self.set_image_orientation_shim(val);
        self
    }
    #[cfg(feature = "PremultiplyAlpha")]
    #[doc = "Change the `premultiplyAlpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`, `PremultiplyAlpha`*"]
    pub fn premultiply_alpha(&mut self, val: PremultiplyAlpha) -> &mut Self {
        self.set_premultiply_alpha_shim(val);
        self
    }
    #[doc = "Change the `resizeHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`*"]
    pub fn resize_height(&mut self, val: u32) -> &mut Self {
        self.set_resize_height_shim(val);
        self
    }
    #[cfg(feature = "ResizeQuality")]
    #[doc = "Change the `resizeQuality` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`, `ResizeQuality`*"]
    pub fn resize_quality(&mut self, val: ResizeQuality) -> &mut Self {
        self.set_resize_quality_shim(val);
        self
    }
    #[doc = "Change the `resizeWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`*"]
    pub fn resize_width(&mut self, val: u32) -> &mut Self {
        self.set_resize_width_shim(val);
        self
    }
}
impl Default for ImageBitmapOptions {
    fn default() -> Self {
        Self::new()
    }
}
