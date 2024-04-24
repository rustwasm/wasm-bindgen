#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = AudioDataCopyToOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioDataCopyToOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDataCopyToOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type AudioDataCopyToOptions;
    #[cfg(feature = "AudioSampleFormat")]
    #[wasm_bindgen(method, getter = "format")]
    fn format_shim(this: &AudioDataCopyToOptions) -> AudioSampleFormat;
    #[cfg(feature = "AudioSampleFormat")]
    #[wasm_bindgen(method, setter = "format")]
    fn set_format_shim(this: &AudioDataCopyToOptions, val: AudioSampleFormat);
    #[wasm_bindgen(method, getter = "frameCount")]
    fn frame_count_shim(this: &AudioDataCopyToOptions) -> u32;
    #[wasm_bindgen(method, setter = "frameCount")]
    fn set_frame_count_shim(this: &AudioDataCopyToOptions, val: u32);
    #[wasm_bindgen(method, getter = "frameOffset")]
    fn frame_offset_shim(this: &AudioDataCopyToOptions) -> u32;
    #[wasm_bindgen(method, setter = "frameOffset")]
    fn set_frame_offset_shim(this: &AudioDataCopyToOptions, val: u32);
    #[wasm_bindgen(method, getter = "planeIndex")]
    fn plane_index_shim(this: &AudioDataCopyToOptions) -> u32;
    #[wasm_bindgen(method, setter = "planeIndex")]
    fn set_plane_index_shim(this: &AudioDataCopyToOptions, val: u32);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `AudioDataCopyToOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `AudioDataCopyToOptions`*"]
pub trait AudioDataCopyToOptionsGetters {
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AudioSampleFormat")]
    #[doc = "Get the `format` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDataCopyToOptions`, `AudioSampleFormat`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn format(&self) -> AudioSampleFormat;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `frameCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDataCopyToOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn frame_count(&self) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `frameOffset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDataCopyToOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn frame_offset(&self) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `planeIndex` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDataCopyToOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn plane_index(&self) -> u32;
}
#[cfg(web_sys_unstable_apis)]
impl AudioDataCopyToOptionsGetters for AudioDataCopyToOptions {
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AudioSampleFormat")]
    fn format(&self) -> AudioSampleFormat {
        self.format_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn frame_count(&self) -> u32 {
        self.frame_count_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn frame_offset(&self) -> u32 {
        self.frame_offset_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn plane_index(&self) -> u32 {
        self.plane_index_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl AudioDataCopyToOptions {
    #[doc = "Construct a new `AudioDataCopyToOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDataCopyToOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(plane_index: u32) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        Self::plane_index(&mut ret, plane_index);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AudioSampleFormat")]
    #[doc = "Change the `format` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDataCopyToOptions`, `AudioSampleFormat`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn format(&mut self, val: AudioSampleFormat) -> &mut Self {
        self.set_format_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `frameCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDataCopyToOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn frame_count(&mut self, val: u32) -> &mut Self {
        self.set_frame_count_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `frameOffset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDataCopyToOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn frame_offset(&mut self, val: u32) -> &mut Self {
        self.set_frame_offset_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `planeIndex` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDataCopyToOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn plane_index(&mut self, val: u32) -> &mut Self {
        self.set_plane_index_shim(val);
        self
    }
}
