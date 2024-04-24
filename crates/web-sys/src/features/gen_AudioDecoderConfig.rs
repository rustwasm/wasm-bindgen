#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = AudioDecoderConfig)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioDecoderConfig` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDecoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type AudioDecoderConfig;
    #[wasm_bindgen(method, getter = "codec")]
    fn codec_shim(this: &AudioDecoderConfig) -> &str;
    #[wasm_bindgen(method, setter = "codec")]
    fn set_codec_shim(this: &AudioDecoderConfig, val: &str);
    #[wasm_bindgen(method, getter = "description")]
    fn description_shim(this: &AudioDecoderConfig) -> &::js_sys::Object;
    #[wasm_bindgen(method, setter = "description")]
    fn set_description_shim(this: &AudioDecoderConfig, val: &::js_sys::Object);
    #[wasm_bindgen(method, getter = "numberOfChannels")]
    fn number_of_channels_shim(this: &AudioDecoderConfig) -> u32;
    #[wasm_bindgen(method, setter = "numberOfChannels")]
    fn set_number_of_channels_shim(this: &AudioDecoderConfig, val: u32);
    #[wasm_bindgen(method, getter = "sampleRate")]
    fn sample_rate_shim(this: &AudioDecoderConfig) -> u32;
    #[wasm_bindgen(method, setter = "sampleRate")]
    fn set_sample_rate_shim(this: &AudioDecoderConfig, val: u32);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `AudioDecoderConfig` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `AudioDecoderConfig`*"]
pub trait AudioDecoderConfigGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `codec` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDecoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn codec(&self) -> &str;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `description` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDecoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn description(&self) -> &::js_sys::Object;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `numberOfChannels` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDecoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn number_of_channels(&self) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `sampleRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDecoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn sample_rate(&self) -> u32;
}
#[cfg(web_sys_unstable_apis)]
impl AudioDecoderConfigGetters for AudioDecoderConfig {
    #[cfg(web_sys_unstable_apis)]
    fn codec(&self) -> &str {
        self.codec_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn description(&self) -> &::js_sys::Object {
        self.description_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn number_of_channels(&self) -> u32 {
        self.number_of_channels_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn sample_rate(&self) -> u32 {
        self.sample_rate_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl AudioDecoderConfig {
    #[doc = "Construct a new `AudioDecoderConfig`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDecoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(codec: &str, number_of_channels: u32, sample_rate: u32) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.codec(codec);
        ret.number_of_channels(number_of_channels);
        ret.sample_rate(sample_rate);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `codec` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDecoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn codec(&mut self, val: &str) -> &mut Self {
        self.set_codec_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `description` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDecoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn description(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.set_description_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `numberOfChannels` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDecoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn number_of_channels(&mut self, val: u32) -> &mut Self {
        self.set_number_of_channels_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `sampleRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDecoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn sample_rate(&mut self, val: u32) -> &mut Self {
        self.set_sample_rate_shim(val);
        self
    }
}
