#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = AudioConfiguration)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioConfiguration` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioConfiguration`*"]
    pub type AudioConfiguration;
    #[wasm_bindgen(method, getter = "bitrate")]
    fn bitrate_shim(this: &AudioConfiguration) -> f64;
    #[wasm_bindgen(method, setter = "bitrate")]
    fn set_bitrate_shim(this: &AudioConfiguration, val: f64);
    #[wasm_bindgen(method, getter = "channels")]
    fn channels_shim(this: &AudioConfiguration) -> String;
    #[wasm_bindgen(method, setter = "channels")]
    fn set_channels_shim(this: &AudioConfiguration, val: &str);
    #[wasm_bindgen(method, getter = "contentType")]
    fn content_type_shim(this: &AudioConfiguration) -> String;
    #[wasm_bindgen(method, setter = "contentType")]
    fn set_content_type_shim(this: &AudioConfiguration, val: &str);
    #[wasm_bindgen(method, getter = "samplerate")]
    fn samplerate_shim(this: &AudioConfiguration) -> u32;
    #[wasm_bindgen(method, setter = "samplerate")]
    fn set_samplerate_shim(this: &AudioConfiguration, val: u32);
}
#[doc = "The trait to access properties on the `AudioConfiguration` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `AudioConfiguration`*"]
pub trait AudioConfigurationGetters {
    #[doc = "Get the `bitrate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioConfiguration`*"]
    fn bitrate(&self) -> f64;
    #[doc = "Get the `channels` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioConfiguration`*"]
    fn channels(&self) -> String;
    #[doc = "Get the `contentType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioConfiguration`*"]
    fn content_type(&self) -> String;
    #[doc = "Get the `samplerate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioConfiguration`*"]
    fn samplerate(&self) -> u32;
}
impl AudioConfigurationGetters for AudioConfiguration {
    fn bitrate(&self) -> f64 {
        self.bitrate_shim()
    }
    fn channels(&self) -> String {
        self.channels_shim()
    }
    fn content_type(&self) -> String {
        self.content_type_shim()
    }
    fn samplerate(&self) -> u32 {
        self.samplerate_shim()
    }
}
impl AudioConfiguration {
    #[doc = "Construct a new `AudioConfiguration`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioConfiguration`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bitrate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioConfiguration`*"]
    pub fn bitrate(&mut self, val: f64) -> &mut Self {
        self.set_bitrate_shim(val);
        self
    }
    #[doc = "Change the `channels` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioConfiguration`*"]
    pub fn channels(&mut self, val: &str) -> &mut Self {
        self.set_channels_shim(val);
        self
    }
    #[doc = "Change the `contentType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioConfiguration`*"]
    pub fn content_type(&mut self, val: &str) -> &mut Self {
        self.set_content_type_shim(val);
        self
    }
    #[doc = "Change the `samplerate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioConfiguration`*"]
    pub fn samplerate(&mut self, val: u32) -> &mut Self {
        self.set_samplerate_shim(val);
        self
    }
}
impl Default for AudioConfiguration {
    fn default() -> Self {
        Self::new()
    }
}
