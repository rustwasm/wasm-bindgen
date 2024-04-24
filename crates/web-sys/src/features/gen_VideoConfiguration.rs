#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = VideoConfiguration)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VideoConfiguration` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoConfiguration`*"]
    pub type VideoConfiguration;
    #[wasm_bindgen(method, getter = "bitrate")]
    fn bitrate_shim(this: &VideoConfiguration) -> f64;
    #[wasm_bindgen(method, setter = "bitrate")]
    fn set_bitrate_shim(this: &VideoConfiguration, val: f64);
    #[wasm_bindgen(method, getter = "contentType")]
    fn content_type_shim(this: &VideoConfiguration) -> String;
    #[wasm_bindgen(method, setter = "contentType")]
    fn set_content_type_shim(this: &VideoConfiguration, val: &str);
    #[wasm_bindgen(method, getter = "framerate")]
    fn framerate_shim(this: &VideoConfiguration) -> String;
    #[wasm_bindgen(method, setter = "framerate")]
    fn set_framerate_shim(this: &VideoConfiguration, val: &str);
    #[wasm_bindgen(method, getter = "height")]
    fn height_shim(this: &VideoConfiguration) -> u32;
    #[wasm_bindgen(method, setter = "height")]
    fn set_height_shim(this: &VideoConfiguration, val: u32);
    #[wasm_bindgen(method, getter = "width")]
    fn width_shim(this: &VideoConfiguration) -> u32;
    #[wasm_bindgen(method, setter = "width")]
    fn set_width_shim(this: &VideoConfiguration, val: u32);
}
#[doc = "The trait to access properties on the `VideoConfiguration` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `VideoConfiguration`*"]
pub trait VideoConfigurationGetters {
    #[doc = "Get the `bitrate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoConfiguration`*"]
    fn bitrate(&self) -> f64;
    #[doc = "Get the `contentType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoConfiguration`*"]
    fn content_type(&self) -> String;
    #[doc = "Get the `framerate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoConfiguration`*"]
    fn framerate(&self) -> String;
    #[doc = "Get the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoConfiguration`*"]
    fn height(&self) -> u32;
    #[doc = "Get the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoConfiguration`*"]
    fn width(&self) -> u32;
}
impl VideoConfigurationGetters for VideoConfiguration {
    fn bitrate(&self) -> f64 {
        self.bitrate_shim()
    }
    fn content_type(&self) -> String {
        self.content_type_shim()
    }
    fn framerate(&self) -> String {
        self.framerate_shim()
    }
    fn height(&self) -> u32 {
        self.height_shim()
    }
    fn width(&self) -> u32 {
        self.width_shim()
    }
}
impl VideoConfiguration {
    #[doc = "Construct a new `VideoConfiguration`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoConfiguration`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bitrate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoConfiguration`*"]
    pub fn bitrate(&mut self, val: f64) -> &mut Self {
        self.set_bitrate_shim(val);
        self
    }
    #[doc = "Change the `contentType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoConfiguration`*"]
    pub fn content_type(&mut self, val: &str) -> &mut Self {
        self.set_content_type_shim(val);
        self
    }
    #[doc = "Change the `framerate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoConfiguration`*"]
    pub fn framerate(&mut self, val: &str) -> &mut Self {
        self.set_framerate_shim(val);
        self
    }
    #[doc = "Change the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoConfiguration`*"]
    pub fn height(&mut self, val: u32) -> &mut Self {
        self.set_height_shim(val);
        self
    }
    #[doc = "Change the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoConfiguration`*"]
    pub fn width(&mut self, val: u32) -> &mut Self {
        self.set_width_shim(val);
        self
    }
}
impl Default for VideoConfiguration {
    fn default() -> Self {
        Self::new()
    }
}
