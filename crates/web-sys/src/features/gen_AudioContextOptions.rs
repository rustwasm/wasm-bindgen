#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = AudioContextOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioContextOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContextOptions`*"]
    pub type AudioContextOptions;
    #[wasm_bindgen(method, getter = "latencyHint")]
    fn latency_hint_shim(this: &AudioContextOptions) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "latencyHint")]
    fn set_latency_hint_shim(this: &AudioContextOptions, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "sampleRate")]
    fn sample_rate_shim(this: &AudioContextOptions) -> f32;
    #[wasm_bindgen(method, setter = "sampleRate")]
    fn set_sample_rate_shim(this: &AudioContextOptions, val: f32);
    #[wasm_bindgen(method, getter = "sinkId")]
    fn sink_id_shim(this: &AudioContextOptions) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "sinkId")]
    fn set_sink_id_shim(this: &AudioContextOptions, val: &::wasm_bindgen::JsValue);
}
#[doc = "The trait to access properties on the `AudioContextOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `AudioContextOptions`*"]
pub trait AudioContextOptionsGetters {
    #[doc = "Get the `latencyHint` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContextOptions`*"]
    fn latency_hint(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `sampleRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContextOptions`*"]
    fn sample_rate(&self) -> f32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `sinkId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContextOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn sink_id(&self) -> &::wasm_bindgen::JsValue;
}
impl AudioContextOptionsGetters for AudioContextOptions {
    fn latency_hint(&self) -> &::wasm_bindgen::JsValue {
        self.latency_hint_shim()
    }
    fn sample_rate(&self) -> f32 {
        self.sample_rate_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn sink_id(&self) -> &::wasm_bindgen::JsValue {
        self.sink_id_shim()
    }
}
impl AudioContextOptions {
    #[doc = "Construct a new `AudioContextOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContextOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `latencyHint` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContextOptions`*"]
    pub fn latency_hint(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_latency_hint_shim(val);
        self
    }
    #[doc = "Change the `sampleRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContextOptions`*"]
    pub fn sample_rate(&mut self, val: f32) -> &mut Self {
        self.set_sample_rate_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `sinkId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContextOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn sink_id(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_sink_id_shim(val);
        self
    }
}
impl Default for AudioContextOptions {
    fn default() -> Self {
        Self::new()
    }
}
