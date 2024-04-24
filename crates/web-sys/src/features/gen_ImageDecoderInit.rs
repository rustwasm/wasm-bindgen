#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ImageDecoderInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ImageDecoderInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageDecoderInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type ImageDecoderInit;
    #[cfg(feature = "ColorSpaceConversion")]
    #[wasm_bindgen(method, getter = "colorSpaceConversion")]
    fn color_space_conversion_shim(this: &ImageDecoderInit) -> ColorSpaceConversion;
    #[cfg(feature = "ColorSpaceConversion")]
    #[wasm_bindgen(method, setter = "colorSpaceConversion")]
    fn set_color_space_conversion_shim(this: &ImageDecoderInit, val: ColorSpaceConversion);
    #[wasm_bindgen(method, getter = "data")]
    fn data_shim(this: &ImageDecoderInit) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "data")]
    fn set_data_shim(this: &ImageDecoderInit, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "desiredHeight")]
    fn desired_height_shim(this: &ImageDecoderInit) -> u32;
    #[wasm_bindgen(method, setter = "desiredHeight")]
    fn set_desired_height_shim(this: &ImageDecoderInit, val: u32);
    #[wasm_bindgen(method, getter = "desiredWidth")]
    fn desired_width_shim(this: &ImageDecoderInit) -> u32;
    #[wasm_bindgen(method, setter = "desiredWidth")]
    fn set_desired_width_shim(this: &ImageDecoderInit, val: u32);
    #[wasm_bindgen(method, getter = "preferAnimation")]
    fn prefer_animation_shim(this: &ImageDecoderInit) -> bool;
    #[wasm_bindgen(method, setter = "preferAnimation")]
    fn set_prefer_animation_shim(this: &ImageDecoderInit, val: bool);
    #[cfg(feature = "PremultiplyAlpha")]
    #[wasm_bindgen(method, getter = "premultiplyAlpha")]
    fn premultiply_alpha_shim(this: &ImageDecoderInit) -> PremultiplyAlpha;
    #[cfg(feature = "PremultiplyAlpha")]
    #[wasm_bindgen(method, setter = "premultiplyAlpha")]
    fn set_premultiply_alpha_shim(this: &ImageDecoderInit, val: PremultiplyAlpha);
    #[wasm_bindgen(method, getter = "type")]
    fn type__shim(this: &ImageDecoderInit) -> &str;
    #[wasm_bindgen(method, setter = "type")]
    fn set_type__shim(this: &ImageDecoderInit, val: &str);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `ImageDecoderInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ImageDecoderInit`*"]
pub trait ImageDecoderInitGetters {
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ColorSpaceConversion")]
    #[doc = "Get the `colorSpaceConversion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ColorSpaceConversion`, `ImageDecoderInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn color_space_conversion(&self) -> ColorSpaceConversion;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageDecoderInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn data(&self) -> &::wasm_bindgen::JsValue;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `desiredHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageDecoderInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn desired_height(&self) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `desiredWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageDecoderInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn desired_width(&self) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `preferAnimation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageDecoderInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn prefer_animation(&self) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "PremultiplyAlpha")]
    #[doc = "Get the `premultiplyAlpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageDecoderInit`, `PremultiplyAlpha`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn premultiply_alpha(&self) -> PremultiplyAlpha;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageDecoderInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn type_(&self) -> &str;
}
#[cfg(web_sys_unstable_apis)]
impl ImageDecoderInitGetters for ImageDecoderInit {
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ColorSpaceConversion")]
    fn color_space_conversion(&self) -> ColorSpaceConversion {
        self.color_space_conversion_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn data(&self) -> &::wasm_bindgen::JsValue {
        self.data_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn desired_height(&self) -> u32 {
        self.desired_height_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn desired_width(&self) -> u32 {
        self.desired_width_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn prefer_animation(&self) -> bool {
        self.prefer_animation_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "PremultiplyAlpha")]
    fn premultiply_alpha(&self) -> PremultiplyAlpha {
        self.premultiply_alpha_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn type_(&self) -> &str {
        self.type__shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl ImageDecoderInit {
    #[doc = "Construct a new `ImageDecoderInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageDecoderInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(data: &::wasm_bindgen::JsValue, type_: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.data(data);
        ret.type_(type_);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ColorSpaceConversion")]
    #[doc = "Change the `colorSpaceConversion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ColorSpaceConversion`, `ImageDecoderInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn color_space_conversion(&mut self, val: ColorSpaceConversion) -> &mut Self {
        self.set_color_space_conversion_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageDecoderInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn data(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_data_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `desiredHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageDecoderInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn desired_height(&mut self, val: u32) -> &mut Self {
        self.set_desired_height_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `desiredWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageDecoderInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn desired_width(&mut self, val: u32) -> &mut Self {
        self.set_desired_width_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `preferAnimation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageDecoderInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn prefer_animation(&mut self, val: bool) -> &mut Self {
        self.set_prefer_animation_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "PremultiplyAlpha")]
    #[doc = "Change the `premultiplyAlpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageDecoderInit`, `PremultiplyAlpha`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn premultiply_alpha(&mut self, val: PremultiplyAlpha) -> &mut Self {
        self.set_premultiply_alpha_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageDecoderInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn type_(&mut self, val: &str) -> &mut Self {
        self.set_type__shim(val);
        self
    }
}
