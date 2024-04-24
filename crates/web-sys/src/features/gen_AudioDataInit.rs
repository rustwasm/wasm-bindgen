#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = AudioDataInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioDataInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDataInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type AudioDataInit;
    #[wasm_bindgen(method, getter = "data")]
    fn data_shim(this: &AudioDataInit) -> ::js_sys::Object;
    #[wasm_bindgen(method, setter = "data")]
    fn set_data_shim(this: &AudioDataInit, val: &::js_sys::Object);
    #[cfg(feature = "AudioSampleFormat")]
    #[wasm_bindgen(method, getter = "format")]
    fn format_shim(this: &AudioDataInit) -> AudioSampleFormat;
    #[cfg(feature = "AudioSampleFormat")]
    #[wasm_bindgen(method, setter = "format")]
    fn set_format_shim(this: &AudioDataInit, val: AudioSampleFormat);
    #[wasm_bindgen(method, getter = "numberOfChannels")]
    fn number_of_channels_shim(this: &AudioDataInit) -> u32;
    #[wasm_bindgen(method, setter = "numberOfChannels")]
    fn set_number_of_channels_shim(this: &AudioDataInit, val: u32);
    #[wasm_bindgen(method, getter = "numberOfFrames")]
    fn number_of_frames_shim(this: &AudioDataInit) -> u32;
    #[wasm_bindgen(method, setter = "numberOfFrames")]
    fn set_number_of_frames_shim(this: &AudioDataInit, val: u32);
    #[wasm_bindgen(method, getter = "sampleRate")]
    fn sample_rate_shim(this: &AudioDataInit) -> f32;
    #[wasm_bindgen(method, setter = "sampleRate")]
    fn set_sample_rate_shim(this: &AudioDataInit, val: f32);
    #[wasm_bindgen(method, getter = "timestamp")]
    fn timestamp_shim(this: &AudioDataInit) -> f64;
    #[wasm_bindgen(method, setter = "timestamp")]
    fn set_timestamp_shim(this: &AudioDataInit, val: f64);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `AudioDataInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `AudioDataInit`*"]
pub trait AudioDataInitGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDataInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn data(&self) -> ::js_sys::Object;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AudioSampleFormat")]
    #[doc = "Get the `format` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDataInit`, `AudioSampleFormat`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn format(&self) -> AudioSampleFormat;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `numberOfChannels` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDataInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn number_of_channels(&self) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `numberOfFrames` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDataInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn number_of_frames(&self) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `sampleRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDataInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn sample_rate(&self) -> f32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDataInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn timestamp(&self) -> f64;
}
#[cfg(web_sys_unstable_apis)]
impl AudioDataInitGetters for AudioDataInit {
    #[cfg(web_sys_unstable_apis)]
    fn data(&self) -> ::js_sys::Object {
        self.data_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AudioSampleFormat")]
    fn format(&self) -> AudioSampleFormat {
        self.format_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn number_of_channels(&self) -> u32 {
        self.number_of_channels_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn number_of_frames(&self) -> u32 {
        self.number_of_frames_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn sample_rate(&self) -> f32 {
        self.sample_rate_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn timestamp(&self) -> f64 {
        self.timestamp_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl AudioDataInit {
    #[cfg(feature = "AudioSampleFormat")]
    #[doc = "Construct a new `AudioDataInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDataInit`, `AudioSampleFormat`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(
        data: &::js_sys::Object,
        format: AudioSampleFormat,
        number_of_channels: u32,
        number_of_frames: u32,
        sample_rate: f32,
        timestamp: f64,
    ) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        Self::data(&mut ret, data);
        Self::format(&mut ret, format);
        Self::number_of_channels(&mut ret, number_of_channels);
        Self::number_of_frames(&mut ret, number_of_frames);
        Self::sample_rate(&mut ret, sample_rate);
        Self::timestamp(&mut ret, timestamp);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDataInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn data(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.set_data_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AudioSampleFormat")]
    #[doc = "Change the `format` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDataInit`, `AudioSampleFormat`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn format(&mut self, val: AudioSampleFormat) -> &mut Self {
        self.set_format_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `numberOfChannels` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDataInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn number_of_channels(&mut self, val: u32) -> &mut Self {
        self.set_number_of_channels_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `numberOfFrames` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDataInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn number_of_frames(&mut self, val: u32) -> &mut Self {
        self.set_number_of_frames_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `sampleRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDataInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn sample_rate(&mut self, val: f32) -> &mut Self {
        self.set_sample_rate_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDataInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn timestamp(&mut self, val: f64) -> &mut Self {
        self.set_timestamp_shim(val);
        self
    }
}
