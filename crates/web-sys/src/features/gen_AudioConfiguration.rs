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
    #[wasm_bindgen(method, setter = "bitrate")]
    fn bitrate_shim(this: &AudioConfiguration, val: f64);
    #[wasm_bindgen(method, setter = "channels")]
    fn channels_shim(this: &AudioConfiguration, val: &str);
    #[wasm_bindgen(method, setter = "contentType")]
    fn content_type_shim(this: &AudioConfiguration, val: &str);
    #[wasm_bindgen(method, setter = "samplerate")]
    fn samplerate_shim(this: &AudioConfiguration, val: u32);
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
        self.bitrate_shim(val);
        self
    }
    #[doc = "Change the `channels` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioConfiguration`*"]
    pub fn channels(&mut self, val: &str) -> &mut Self {
        self.channels_shim(val);
        self
    }
    #[doc = "Change the `contentType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioConfiguration`*"]
    pub fn content_type(&mut self, val: &str) -> &mut Self {
        self.content_type_shim(val);
        self
    }
    #[doc = "Change the `samplerate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioConfiguration`*"]
    pub fn samplerate(&mut self, val: u32) -> &mut Self {
        self.samplerate_shim(val);
        self
    }
}
impl Default for AudioConfiguration {
    fn default() -> Self {
        Self::new()
    }
}
