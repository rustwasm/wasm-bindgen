#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = MediaRecorderOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaRecorderOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorderOptions`*"]
    pub type MediaRecorderOptions;
    #[wasm_bindgen(method, getter = "audioBitsPerSecond")]
    fn audio_bits_per_second_shim(this: &MediaRecorderOptions) -> u32;
    #[wasm_bindgen(method, setter = "audioBitsPerSecond")]
    fn set_audio_bits_per_second_shim(this: &MediaRecorderOptions, val: u32);
    #[wasm_bindgen(method, getter = "bitsPerSecond")]
    fn bits_per_second_shim(this: &MediaRecorderOptions) -> u32;
    #[wasm_bindgen(method, setter = "bitsPerSecond")]
    fn set_bits_per_second_shim(this: &MediaRecorderOptions, val: u32);
    #[wasm_bindgen(method, getter = "mimeType")]
    fn mime_type_shim(this: &MediaRecorderOptions) -> &str;
    #[wasm_bindgen(method, setter = "mimeType")]
    fn set_mime_type_shim(this: &MediaRecorderOptions, val: &str);
    #[wasm_bindgen(method, getter = "videoBitsPerSecond")]
    fn video_bits_per_second_shim(this: &MediaRecorderOptions) -> u32;
    #[wasm_bindgen(method, setter = "videoBitsPerSecond")]
    fn set_video_bits_per_second_shim(this: &MediaRecorderOptions, val: u32);
}
#[doc = "The trait to access properties on the `MediaRecorderOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `MediaRecorderOptions`*"]
pub trait MediaRecorderOptionsGetters {
    #[doc = "Get the `audioBitsPerSecond` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorderOptions`*"]
    fn audio_bits_per_second(&self) -> u32;
    #[doc = "Get the `bitsPerSecond` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorderOptions`*"]
    fn bits_per_second(&self) -> u32;
    #[doc = "Get the `mimeType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorderOptions`*"]
    fn mime_type(&self) -> &str;
    #[doc = "Get the `videoBitsPerSecond` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorderOptions`*"]
    fn video_bits_per_second(&self) -> u32;
}
impl MediaRecorderOptionsGetters for MediaRecorderOptions {
    fn audio_bits_per_second(&self) -> u32 {
        self.audio_bits_per_second_shim()
    }
    fn bits_per_second(&self) -> u32 {
        self.bits_per_second_shim()
    }
    fn mime_type(&self) -> &str {
        self.mime_type_shim()
    }
    fn video_bits_per_second(&self) -> u32 {
        self.video_bits_per_second_shim()
    }
}
impl MediaRecorderOptions {
    #[doc = "Construct a new `MediaRecorderOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorderOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `audioBitsPerSecond` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorderOptions`*"]
    pub fn audio_bits_per_second(&mut self, val: u32) -> &mut Self {
        self.set_audio_bits_per_second_shim(val);
        self
    }
    #[doc = "Change the `bitsPerSecond` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorderOptions`*"]
    pub fn bits_per_second(&mut self, val: u32) -> &mut Self {
        self.set_bits_per_second_shim(val);
        self
    }
    #[doc = "Change the `mimeType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorderOptions`*"]
    pub fn mime_type(&mut self, val: &str) -> &mut Self {
        self.set_mime_type_shim(val);
        self
    }
    #[doc = "Change the `videoBitsPerSecond` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorderOptions`*"]
    pub fn video_bits_per_second(&mut self, val: u32) -> &mut Self {
        self.set_video_bits_per_second_shim(val);
        self
    }
}
impl Default for MediaRecorderOptions {
    fn default() -> Self {
        Self::new()
    }
}
