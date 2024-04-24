#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = AudioEncoderConfig)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioEncoderConfig` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type AudioEncoderConfig;
    #[wasm_bindgen(method, getter = "bitrate")]
    fn bitrate_shim(this: &AudioEncoderConfig) -> f64;
    #[wasm_bindgen(method, setter = "bitrate")]
    fn set_bitrate_shim(this: &AudioEncoderConfig, val: f64);
    #[wasm_bindgen(method, getter = "codec")]
    fn codec_shim(this: &AudioEncoderConfig) -> &str;
    #[wasm_bindgen(method, setter = "codec")]
    fn set_codec_shim(this: &AudioEncoderConfig, val: &str);
    #[wasm_bindgen(method, getter = "numberOfChannels")]
    fn number_of_channels_shim(this: &AudioEncoderConfig) -> u32;
    #[wasm_bindgen(method, setter = "numberOfChannels")]
    fn set_number_of_channels_shim(this: &AudioEncoderConfig, val: u32);
    #[wasm_bindgen(method, getter = "sampleRate")]
    fn sample_rate_shim(this: &AudioEncoderConfig) -> u32;
    #[wasm_bindgen(method, setter = "sampleRate")]
    fn set_sample_rate_shim(this: &AudioEncoderConfig, val: u32);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `AudioEncoderConfig` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `AudioEncoderConfig`*"]
pub trait AudioEncoderConfigGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `bitrate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn bitrate(&self) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `codec` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn codec(&self) -> &str;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `numberOfChannels` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn number_of_channels(&self) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `sampleRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn sample_rate(&self) -> u32;
}
#[cfg(web_sys_unstable_apis)]
impl AudioEncoderConfigGetters for AudioEncoderConfig {
    #[cfg(web_sys_unstable_apis)]
    fn bitrate(&self) -> f64 {
        self.bitrate_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn codec(&self) -> &str {
        self.codec_shim()
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
impl AudioEncoderConfig {
    #[doc = "Construct a new `AudioEncoderConfig`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(codec: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.codec(codec);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `bitrate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn bitrate(&mut self, val: f64) -> &mut Self {
        self.set_bitrate_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `codec` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn codec(&mut self, val: &str) -> &mut Self {
        self.set_codec_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `numberOfChannels` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioEncoderConfig`*"]
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
    #[doc = "*This API requires the following crate features to be activated: `AudioEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn sample_rate(&mut self, val: u32) -> &mut Self {
        self.set_sample_rate_shim(val);
        self
    }
}
