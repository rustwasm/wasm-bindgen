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
    #[wasm_bindgen(method, setter = "buffer")]
    fn buffer_shim(this: &AudioBufferSourceOptions, val: Option<&AudioBuffer>);
    #[wasm_bindgen(method, setter = "detune")]
    fn detune_shim(this: &AudioBufferSourceOptions, val: f32);
    #[wasm_bindgen(method, setter = "loop")]
    fn loop__shim(this: &AudioBufferSourceOptions, val: bool);
    #[wasm_bindgen(method, setter = "loopEnd")]
    fn loop_end_shim(this: &AudioBufferSourceOptions, val: f64);
    #[wasm_bindgen(method, setter = "loopStart")]
    fn loop_start_shim(this: &AudioBufferSourceOptions, val: f64);
    #[wasm_bindgen(method, setter = "playbackRate")]
    fn playback_rate_shim(this: &AudioBufferSourceOptions, val: f32);
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
        self.buffer_shim(val);
        self
    }
    #[doc = "Change the `detune` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceOptions`*"]
    pub fn detune(&mut self, val: f32) -> &mut Self {
        self.detune_shim(val);
        self
    }
    #[doc = "Change the `loop` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceOptions`*"]
    pub fn loop_(&mut self, val: bool) -> &mut Self {
        self.loop__shim(val);
        self
    }
    #[doc = "Change the `loopEnd` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceOptions`*"]
    pub fn loop_end(&mut self, val: f64) -> &mut Self {
        self.loop_end_shim(val);
        self
    }
    #[doc = "Change the `loopStart` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceOptions`*"]
    pub fn loop_start(&mut self, val: f64) -> &mut Self {
        self.loop_start_shim(val);
        self
    }
    #[doc = "Change the `playbackRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceOptions`*"]
    pub fn playback_rate(&mut self, val: f32) -> &mut Self {
        self.playback_rate_shim(val);
        self
    }
}
impl Default for AudioBufferSourceOptions {
    fn default() -> Self {
        Self::new()
    }
}
