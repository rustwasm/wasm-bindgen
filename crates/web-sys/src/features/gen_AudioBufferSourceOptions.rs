#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = AudioBufferSourceOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioBufferSourceOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceOptions`*"]
    pub type AudioBufferSourceOptions;
    #[cfg(feature = "AudioBuffer")]
    #[wasm_bindgen(method, getter = "buffer")]
    fn buffer_shim(this: &AudioBufferSourceOptions) -> Option<AudioBuffer>;
    #[cfg(feature = "AudioBuffer")]
    #[wasm_bindgen(method, setter = "buffer")]
    fn set_buffer_shim(this: &AudioBufferSourceOptions, val: Option<&AudioBuffer>);
    #[wasm_bindgen(method, getter = "detune")]
    fn detune_shim(this: &AudioBufferSourceOptions) -> f32;
    #[wasm_bindgen(method, setter = "detune")]
    fn set_detune_shim(this: &AudioBufferSourceOptions, val: f32);
    #[wasm_bindgen(method, getter = "loop")]
    fn loop__shim(this: &AudioBufferSourceOptions) -> bool;
    #[wasm_bindgen(method, setter = "loop")]
    fn set_loop__shim(this: &AudioBufferSourceOptions, val: bool);
    #[wasm_bindgen(method, getter = "loopEnd")]
    fn loop_end_shim(this: &AudioBufferSourceOptions) -> f64;
    #[wasm_bindgen(method, setter = "loopEnd")]
    fn set_loop_end_shim(this: &AudioBufferSourceOptions, val: f64);
    #[wasm_bindgen(method, getter = "loopStart")]
    fn loop_start_shim(this: &AudioBufferSourceOptions) -> f64;
    #[wasm_bindgen(method, setter = "loopStart")]
    fn set_loop_start_shim(this: &AudioBufferSourceOptions, val: f64);
    #[wasm_bindgen(method, getter = "playbackRate")]
    fn playback_rate_shim(this: &AudioBufferSourceOptions) -> f32;
    #[wasm_bindgen(method, setter = "playbackRate")]
    fn set_playback_rate_shim(this: &AudioBufferSourceOptions, val: f32);
}
#[doc = "The trait to access properties on the `AudioBufferSourceOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceOptions`*"]
pub trait AudioBufferSourceOptionsGetters {
    #[cfg(feature = "AudioBuffer")]
    #[doc = "Get the `buffer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`, `AudioBufferSourceOptions`*"]
    fn buffer(&self) -> Option<AudioBuffer>;
    #[doc = "Get the `detune` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceOptions`*"]
    fn detune(&self) -> f32;
    #[doc = "Get the `loop` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceOptions`*"]
    fn loop_(&self) -> bool;
    #[doc = "Get the `loopEnd` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceOptions`*"]
    fn loop_end(&self) -> f64;
    #[doc = "Get the `loopStart` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceOptions`*"]
    fn loop_start(&self) -> f64;
    #[doc = "Get the `playbackRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceOptions`*"]
    fn playback_rate(&self) -> f32;
}
impl AudioBufferSourceOptionsGetters for AudioBufferSourceOptions {
    #[cfg(feature = "AudioBuffer")]
    fn buffer(&self) -> Option<AudioBuffer> {
        self.buffer_shim()
    }
    fn detune(&self) -> f32 {
        self.detune_shim()
    }
    fn loop_(&self) -> bool {
        self.loop__shim()
    }
    fn loop_end(&self) -> f64 {
        self.loop_end_shim()
    }
    fn loop_start(&self) -> f64 {
        self.loop_start_shim()
    }
    fn playback_rate(&self) -> f32 {
        self.playback_rate_shim()
    }
}
impl AudioBufferSourceOptions {
    #[doc = "Construct a new `AudioBufferSourceOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "AudioBuffer")]
    #[doc = "Change the `buffer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`, `AudioBufferSourceOptions`*"]
    pub fn buffer(&mut self, val: Option<&AudioBuffer>) -> &mut Self {
        self.set_buffer_shim(val);
        self
    }
    #[doc = "Change the `detune` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceOptions`*"]
    pub fn detune(&mut self, val: f32) -> &mut Self {
        self.set_detune_shim(val);
        self
    }
    #[doc = "Change the `loop` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceOptions`*"]
    pub fn loop_(&mut self, val: bool) -> &mut Self {
        self.set_loop__shim(val);
        self
    }
    #[doc = "Change the `loopEnd` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceOptions`*"]
    pub fn loop_end(&mut self, val: f64) -> &mut Self {
        self.set_loop_end_shim(val);
        self
    }
    #[doc = "Change the `loopStart` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceOptions`*"]
    pub fn loop_start(&mut self, val: f64) -> &mut Self {
        self.set_loop_start_shim(val);
        self
    }
    #[doc = "Change the `playbackRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceOptions`*"]
    pub fn playback_rate(&mut self, val: f32) -> &mut Self {
        self.set_playback_rate_shim(val);
        self
    }
}
impl Default for AudioBufferSourceOptions {
    fn default() -> Self {
        Self::new()
    }
}
